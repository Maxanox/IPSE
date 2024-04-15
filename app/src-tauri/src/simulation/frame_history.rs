use std::collections::VecDeque;

/// A data structure that represents a frame history.
///
/// This struct is used to store a history of frames with a specified duration and frame rate.
/// It keeps track of the data in a `VecDeque` and automatically removes the oldest frame
/// when the capacity is reached.
///
/// # Type Parameters
///
/// - `T`: The type of data to store in the frame history.
pub struct FrameHistory<T> {
    datas: VecDeque<T>,
    capacity: usize,
    duration: f32,
    delay: f32,
}

impl<T> FrameHistory<T> {
    /// Creates a new `FrameHistory` with the specified duration and frame rate.
    ///
    /// # Parameters
    ///
    /// - `duration`: The duration of the frame history in seconds.
    /// - `frame_per_second`: The frame rate in frames per second.
    ///
    /// # Returns
    ///
    /// A new `FrameHistory` instance.
    pub fn new(duration: f32, frame_per_second: f32) -> Self {
        let capacity = (duration * frame_per_second) as usize;
        FrameHistory {
            datas: VecDeque::with_capacity(capacity),
            capacity,
            duration,
            delay: 1.0 / frame_per_second,
        }
    }

    /// Pushes a new frame to the frame history.
    ///
    /// If the frame history is already at its capacity, the oldest frame will be removed
    /// before adding the new frame.
    ///
    /// # Parameters
    ///
    /// - `value`: The value representing the new frame.
    pub fn push(&mut self, value: T) {
        if self.datas.len() == self.capacity {
            self.datas.pop_front();
        }
        self.datas.push_back(value);
    }
}