use std::{
    mem::{ManuallyDrop, MaybeUninit},
    thread::{JoinHandle, Result},
};

pub struct LocalThread<F, T> {
    res: ManuallyDrop<T>,
    func: ManuallyDrop<F>,
}

impl<F, T> LocalThread<F, T> {
    #[inline]
    pub fn new(func: F) -> Self {
        Self {
            func: ManuallyDrop::new(func),
            res: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }

    pub fn spawn<'a>(&'a mut self) -> LocalJoinHandle<T>
    where
        F: FnOnce() -> T + Send + Sync + 'a,
        T: Send + Sync + 'a,
    {
        let raw_res = Raw::new(&*self.res);
        let raw_func = Raw(&self.func as *const _ as *const _);
        let handle = std::thread::spawn(move || {
            let raw_res = raw_res;
            let raw_func = raw_func;

            unsafe {
                <*mut T as From<_>>::from(raw_res)
                    .write(<*mut F as From<_>>::from(raw_func).read()())
            }
        });

        LocalJoinHandle {
            res: &mut self.res,
            handle: ManuallyDrop::new(handle),
        }
    }
}

pub struct LocalJoinHandle<'a, T> {
    res: &'a mut T,
    handle: ManuallyDrop<JoinHandle<()>>,
}

impl<'a, T> LocalJoinHandle<'a, T> {
    pub fn join(self) -> Result<T> {        
        let join_res = unsafe { core::ptr::read(&*self.handle) }.join();
        let actual_res = unsafe { core::ptr::read(&*self.res) };
        core::mem::forget(self);
        join_res.map(|_| actual_res)
    }
}

impl<'a, T> Drop for LocalJoinHandle<'a, T> {
    fn drop(&mut self) {
        unsafe { core::ptr::read(&*self.handle) }.join().unwrap()
    }
}

#[derive(Clone, Copy)]
struct Raw(*const ());

impl Raw {
    #[inline(always)]
    pub fn new<T>(val: &T) -> Self {
        Self(val as *const T as *const ())
    }
}

impl<T> From<Raw> for *const T {
    #[inline(always)]
    fn from(value: Raw) -> Self {
        value.0 as Self
    }
}

impl<T> From<Raw> for *mut T {
    #[inline(always)]
    fn from(value: Raw) -> Self {
        value.0 as Self
    }
}

unsafe impl Sync for Raw {}
unsafe impl Send for Raw {}
