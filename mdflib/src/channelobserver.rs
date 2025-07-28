//! ChannelObserver wrapper for mdflib
//!
//! This module provides safe Rust wrappers around the mdflib IChannelObserver functionality.

use crate::error::Result;
use mdflib_sys as ffi;
use std::marker::PhantomData;

/// Represents an immutable reference to a channel observer in an MDF file.
///
/// A channel observer holds all sample data for a specific channel and provides
/// methods to access channel values and engineering values.
#[derive(Debug, Clone, Copy)]
pub struct ChannelObserverRef<'a> {
    pub(crate) inner: *const ffi::IChannelObserver,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ChannelObserverRef<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: *const ffi::IChannelObserver) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Gets the number of samples in this channel observer.
    pub fn get_nof_samples(&self) -> usize {
        unsafe { ffi::ChannelObserverGetNofSamples(self.inner) }
    }

    /// Gets the channel value (raw, unscaled) for a specific sample.
    ///
    /// # Arguments
    /// * `sample` - The sample index (0-based)
    ///
    /// # Returns
    /// Returns `Some(value)` if the sample is valid, `None` otherwise.
    pub fn get_channel_value(&self, sample: usize) -> Option<f64> {
        let mut value = 0.0;
        let valid = unsafe { ffi::ChannelObserverGetChannelValue(self.inner, sample, &mut value) };
        if valid {
            Some(value)
        } else {
            None
        }
    }

    /// Gets the engineering value (scaled) for a specific sample.
    ///
    /// # Arguments
    /// * `sample` - The sample index (0-based)
    ///
    /// # Returns
    /// Returns `Some(value)` if the sample is valid, `None` otherwise.
    pub fn get_eng_value(&self, sample: usize) -> Option<f64> {
        let mut value = 0.0;
        let valid = unsafe { ffi::ChannelObserverGetEngValue(self.inner, sample, &mut value) };
        if valid {
            Some(value)
        } else {
            None
        }
    }

    /// Checks if a specific sample is valid.
    ///
    /// # Arguments
    /// * `sample` - The sample index (0-based)
    ///
    /// # Returns
    /// Returns `true` if the sample is valid, `false` otherwise.
    pub fn is_valid(&self, sample: usize) -> bool {
        unsafe { ffi::ChannelObserverGetValid(self.inner, sample) }
    }

    /// Gets all channel values (raw, unscaled) for all samples.
    ///
    /// # Returns
    /// Returns a vector of `Option<f64>` where `None` indicates an invalid sample.
    pub fn get_all_channel_values(&self) -> Vec<Option<f64>> {
        let nof_samples = self.get_nof_samples();
        let mut values = Vec::with_capacity(nof_samples);
        for sample in 0..nof_samples {
            values.push(self.get_channel_value(sample));
        }
        values
    }

    /// Gets all engineering values (scaled) for all samples.
    ///
    /// # Returns
    /// Returns a vector of `Option<f64>` where `None` indicates an invalid sample.
    pub fn get_all_eng_values(&self) -> Vec<Option<f64>> {
        let nof_samples = self.get_nof_samples();
        let mut values = Vec::with_capacity(nof_samples);
        for sample in 0..nof_samples {
            values.push(self.get_eng_value(sample));
        }
        values
    }
}

/// Represents a mutable channel observer in an MDF file.
///
/// This wrapper provides ownership of the underlying IChannelObserver and automatically
/// cleans up resources when dropped.
#[derive(Debug)]
pub struct ChannelObserver<'a> {
    pub(crate) inner: *mut ffi::IChannelObserver,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ChannelObserver<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: *mut ffi::IChannelObserver) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

// Implement Deref to allow using ChannelObserver as ChannelObserverRef
impl<'a> std::ops::Deref for ChannelObserver<'a> {
    type Target = ChannelObserverRef<'a>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const ChannelObserver as *const ChannelObserverRef) }
    }
}

// Implement Drop to clean up the observer when it goes out of scope
impl<'a> Drop for ChannelObserver<'a> {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                ffi::ChannelObserverUnInit(self.inner);
            }
        }
    }
}

// Safety: ChannelObserver can be safely sent between threads
unsafe impl<'a> Send for ChannelObserver<'a> {}
unsafe impl<'a> Sync for ChannelObserver<'a> {}

/// Creates a channel observer for a specific channel in a data group.
///
/// This function creates a channel observer that can be used to read sample data
/// from a channel. The observer holds all sample data for the channel in memory.
///
/// # Safety
///
/// This function is unsafe because it takes raw pointers to C++ objects. The caller
/// must ensure that:
/// - The pointers are valid and point to live objects
/// - The objects remain valid for the lifetime of the observer
/// - The pointers are properly aligned and non-null
///
/// # Arguments
/// * `data_group` - Raw pointer to the data group containing the channel
/// * `channel_group` - Raw pointer to the channel group containing the channel
/// * `channel` - Raw pointer to the specific channel to observe
///
/// # Returns
/// Returns a `Result<ChannelObserver>` if successful, or an error if creation fails.
///
/// # Example
/// ```no_run
/// use mdflib::*;
///
/// # fn example() -> mdflib::Result<()> {
/// let reader = reader::MdfReader::new("example.mf4")?;
/// // ... get data_group, channel_group, and channel from file ...
/// # let file = reader.get_file().unwrap();
/// # let data_group = file.get_data_group(0);
/// # let channel_group = data_group.get_channel_group_by_index(0).unwrap();
/// # let channel = channel_group.get_channel(0).unwrap();
///
/// let observer = unsafe {
///     create_channel_observer(data_group.as_ptr(), channel_group.as_ptr(), channel.as_ptr())?
/// };
/// let nof_samples = observer.get_nof_samples();
///
/// for sample in 0..nof_samples {
///     if let Some(value) = observer.get_eng_value(sample) {
///         println!("Sample {}: {}", sample, value);
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub unsafe fn create_channel_observer<'a>(
    data_group: *const ffi::IDataGroup,
    channel_group: *const ffi::IChannelGroup,
    channel: *const ffi::IChannel,
) -> Result<ChannelObserver<'a>> {
    let observer = unsafe { ffi::CreateChannelObserver(data_group, channel_group, channel) };

    if observer.is_null() {
        return Err(crate::error::MdfError::NullPointer);
    }

    Ok(ChannelObserver::new(observer))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_channel_observer_basic() {
        // Basic test to ensure the module compiles
        // More comprehensive tests would require actual MDF data
    }
}
