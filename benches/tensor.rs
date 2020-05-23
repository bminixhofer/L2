#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;

    use l2::tensor::*;
    #[bench]
    fn bench_allocate_1d_tensor(b: &mut Bencher) {
        b.iter(|| {
            let _t = Tensor::zeros(&[64 * 64]).unwrap();
        });
    }

    #[bench]
    fn bench_slice_1d_tensor_chunk(b: &mut Bencher) {
        let t = Tensor::zeros(&[64 * 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 64]]).unwrap();
        })
    }

    #[bench]
    fn bench_allocate_2d_tensor(b: &mut Bencher) {
        b.iter(|| {
            let _t = Tensor::zeros(&[64, 64]).unwrap();
        });
    }

    #[bench]
    fn bench_slice_2d_tensor_row(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 1], [0, 64]]).unwrap();
        })
    }

    #[bench]
    fn bench_slice_2d_tensor_col(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 64], [0, 1]]).unwrap();
        })
    }

    #[bench]
    fn bench_slice_2d_tensor_col_neg_1(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, -1], [0, 1]]).unwrap();
        })
    }
    #[bench]
    fn bench_slice_2d_tensor_chunk(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 16], [0, 16]]).unwrap();
        })
    }
    #[bench]
    fn bench_allocate_3d_tensor(b: &mut Bencher) {
        b.iter(|| {
            let _t = Tensor::zeros(&[64, 64, 64]).unwrap();
        });
    }
    #[bench]
    fn bench_slice_3d_tensor_row(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 1], [0, 1], [0, 64]]).unwrap();
        })
    }

    #[bench]
    fn bench_slice_3d_tensor_row_neg_1(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 1], [0, 1], [0, -1]]).unwrap();
        })
    }

    #[bench]
    fn bench_slice_3d_tensor_row_automatic_slicing(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 1], [0, 1]]).unwrap();
        })
    }

    #[bench]
    fn bench_slice_3d_tensor_col(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 1], [0, 64], [0, 1]]).unwrap();
        })
    }

    #[bench]
    fn bench_slice_3d_tensor_channel(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 64], [0, 1], [0, 1]]).unwrap();
        })
    }

    #[bench]
    fn bench_slice_3d_tensor_chunk(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 16], [0, 16], [0, 16]]).unwrap();
        })
    }

    #[bench]
    fn bench_allocate_4d_tensor_small(b: &mut Bencher) {
        b.iter(|| {
            let _t = Tensor::zeros(&[16, 16, 16, 16]).unwrap();
        });
    }
    #[bench]
    fn bench_slice_4d_tensor_row(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64, 64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 1], [0, 1], [0, 1], [0, 64]]).unwrap();
        })
    }

    #[bench]
    fn bench_slice_4d_tensor_col(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64, 64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 1], [0, 1], [0, 64], [0, 1]]).unwrap();
        })
    }

    #[bench]
    fn bench_slice_4d_tensor_channel(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64, 64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 1], [0, 64], [0, 1], [0, 1]]).unwrap();
        })
    }

    #[bench]
    fn bench_slice_4d_tensor_batch(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64, 64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 64], [0, 1], [0, 1], [0, 1]]).unwrap();
        })
    }

    #[bench]
    fn bench_slice_4d_tensor_chunk(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64, 64, 64]).unwrap();
        b.iter(|| {
            let _x = t.slice(&[[0, 16], [0, 16], [0, 16], [0, 16]]).unwrap();
        })
    }

    #[bench]
    fn bench_view_2d_to_1d(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64]).unwrap();
        b.iter(|| {
            let _x = t.view(&[64 * 64]).unwrap();
        })
    }

    #[bench]
    fn bench_view_2d_to_1d_automatic_expanding(b: &mut Bencher) {
        let t = Tensor::zeros(&[64, 64]).unwrap();
        b.iter(|| {
            let _x = t.view(&[-1]).unwrap();
        })
    }

    #[bench]
    fn bench_add(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();
        let y = Tensor::zeros(&[256, 256]).unwrap();
        b.iter(|| {
            let _z = &x + &y;
        })
    }

    #[bench]
    fn bench_pow(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.pow(2).unwrap();
        })
    }

    #[bench]
    fn bench_sqrt(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.sqrt().unwrap();
        })
    }

    #[bench]
    fn bench_exp(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.exp().unwrap();
        })
    }

    #[bench]
    fn bench_log(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.log().unwrap();
        })
    }

    #[bench]
    fn bench_log10(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.log10().unwrap();
        })
    }
    #[bench]
    fn bench_abs(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.abs().unwrap();
        })
    }

    #[bench]
    fn bench_sin(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.sin().unwrap();
        })
    }
    #[bench]
    fn bench_cos(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.cos().unwrap();
        })
    }

    #[bench]
    fn bench_tan(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.tan().unwrap();
        })
    }

    #[bench]
    fn bench_sum(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.sum(-1).unwrap();
        })
    }

    #[bench]
    fn bench_mean(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.mean(-1).unwrap();
        })
    }

    #[bench]
    fn bench_max(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.max(-1).unwrap();
        })
    }

    #[bench]
    fn bench_min(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.min(-1).unwrap();
        })
    }
    #[bench]
    fn bench_argmax(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.argmax(-1).unwrap();
        })
    }

    #[bench]
    fn bench_argmin(b: &mut Bencher) {
        let x = Tensor::zeros(&[256, 256]).unwrap();

        b.iter(|| {
            let _y = x.mean(-1).unwrap();
        })
    }
}
