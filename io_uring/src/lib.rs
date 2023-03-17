
// pub fn call(path: String) -> usize {
//     let mut file = fs::File::open(&path).unwrap();
//     // File is splitted into multiple segments, read the offsets such that we can read segments correctly
//     let offsets = read_offsets(&mut file);

//     // settup uring
//     let mut ring = IoUring::new(1024).unwrap();
//     // Read batch_size from the command line argument
//     let batch_size = args.batch_size;

//     let rounds = (offsets.len() - 1) / batch_size;
//     // Pre-allocate the buffer to avoid repeated buffer allocation
//     let mut buffers = vec![Vec::new(); batch_size];

//     let now = std::time::Instant::now();
//     for i in 0..rounds {
//         let base = i * batch_size;
//         batch_read(
//             &file,
//             &mut ring,
//             &mut buffers,
//             &offsets,
//             base,
//             batch_size,
//         )
//     }
// }

// #[allow(clippy::uninit_vec)]
// fn batch_read(
//     file: &fs::File,
//     ring: &mut IoUring,
//     buffers: &mut [Vec<u8>],
//     offsets: &[u64],
//     base: usize,
//     batch_size: usize
// ) {
//     for j in 0..batch_size {
//         let mut submission = ring.submission();
//         let start = offsets[base + j];
//         let end = offsets[base + j + 1];
//         let buf = buffers.get_mut(j).unwrap();
//         let len = (end - start) as usize;
//         buf.clear();
//         buf.reserve(len);
//         unsafe {
//             buf.set_len(len);
//         }

//         let read_e = opcode::Read::new(types::Fd(file.as_raw_fd()), buf.as_mut_ptr(), len as _)
//             .offset64(start as i64)
//             .build();
//         unsafe {
//             submission.push(&read_e).unwrap();
//         }
//     }
//     // Batch submit and wait here
//     ring.submit_and_wait(batch_size).unwrap();
//     let completions = ring.completion();
//     for _ in completions {}
// }

use io_uring::{opcode, types, IoUring};
use std::os::unix::io::AsRawFd;
use std::{fs, io};

fn call(path: String) -> io::Result<()> {
    let mut ring = IoUring::new(8)?;

    let fd = fs::File::open(path)?;
    let mut buf = vec![0; 1024];

    let read_e = opcode::Read::new(types::Fd(fd.as_raw_fd()), buf.as_mut_ptr(), buf.len() as _)
        .build()
        .user_data(0x42);

    // Note that the developer needs to ensure
    // that the entry pushed into submission queue is valid (e.g. fd, buffer).
    unsafe {
        ring.submission()
            .push(&read_e)
            .expect("submission queue is full");
    }

    ring.submit_and_wait(1)?;

    let cqe = ring.completion().next().expect("completion queue is empty");

    assert_eq!(cqe.user_data(), 0x42);
    assert!(cqe.result() >= 0, "read error: {}", cqe.result());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        call("sl.zip".to_owned());
    }
}
