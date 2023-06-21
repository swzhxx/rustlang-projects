use std::{
    marker::PhantomData,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{self, AtomicUsize, Ordering},
};

pub struct Arc<T> {
    ptr: NonNull<ArcInner<T>>,
    phantom: PhantomData<ArcInner<T>>,
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        let boxed = Box::new(ArcInner {
            rc: AtomicUsize::new(1),
            data,
        });
        Arc {
            ptr: NonNull::new(Box::into_raw(boxed)).unwrap(),
            phantom: PhantomData,
        }
    }
}

unsafe impl<T> Send for Arc<T> where T: Sync + Send {}
unsafe impl<T> Sync for Arc<T> where T: Sync + Send {}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = unsafe { self.ptr.as_ref() };
        &inner.data
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Arc<T> {
        let inner = unsafe { self.ptr.as_ref() };
        let old_rc = inner.rc.fetch_add(1, Ordering::Relaxed);
        if old_rc >= isize::MAX as usize {
            std::process::abort()
        }
        Self {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        if inner.rc.fetch_sub(1, Ordering::Relaxed) != 1 {
            return;
        }
        // 我们需要防止针对 inner 的使用和删除的重排序，
        // 因此使用 fence 来进行保护是非常有必要的
        atomic::fence(Ordering::Acquire);
        // 安全保证：我们知道这是最后一个对 ArcInner 的引用，并且这个指针是有效的
        unsafe {
            Box::from_raw(self.ptr.as_ptr());
        }
    }
}

pub struct ArcInner<T> {
    rc: AtomicUsize,
    data: T,
}
