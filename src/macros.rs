// Copyright 2018-2020 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Macros

/// This macro crates a test for send an sync
#[cfg(test)]
#[macro_export]
macro_rules! test_trait_impl {
    ($n:ident, $t:ty) => {
        paste::item! {
            #[test]
            #[allow(non_snake_case)]
            fn [<test_send_ $n>]() {
                fn assert_send<T: Send>() {}
                assert_send::<$t>();
            }
        }

        paste::item! {
            #[test]
            #[allow(non_snake_case)]
            fn [<test_sync_ $n>]() {
                fn assert_sync<T: Sync>() {}
                assert_sync::<$t>();
            }
        }

        paste::item! {
            #[test]
            #[allow(non_snake_case)]
            fn [<test_clone_ $n>]() {
                fn assert_clone<T: Clone>() {}
                assert_clone::<$t>();
            }
        }
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! test_solver_rosenbrock {
    ($n:ident, $solver:expr, $n_iter:expr) => {
        paste::item! {
            #[test]
            #[allow(non_snake_case)]
            fn [<test_rosenbrock_ $n>]() {

               #[derive(Clone, Default, Serialize, Deserialize)]
               struct Rosenbrock {
                   a: f64,
                   b: f64,
               }

               impl ArgminOp for Rosenbrock {
                   type Param = Array1<f64>;
                   type Output = f64;
                   type Hessian = Array2<f64>;
                   type Jacobian = ();

                   fn apply(&self, p: &Self::Param) -> Result<Self::Output, Error> {
                       Ok(rosenbrock(&p.to_vec(), self.a, self.b))
                   }

                   fn gradient(&self, p: &Self::Param) -> Result<Self::Param, Error> {
                       Ok((*p).forward_diff(&|x| rosenbrock(&x.to_vec(), self.a, self.b)))
                   }
               }
                let cost = Rosenbrock { a: 1.0, b: 100.0 };

                // Define initial parameter vector
                let init_param: Array1<f64> = array![-1.2, 1.2];

                let solver = $solver;
                let res = Executor::new(cost, solver, init_param)
                    .max_iters($n_iter)
                    .run().unwrap();
                assert_relative_eq!(res.state.param, array![1.0, 1.0], epsilon=1e-4);

            }
        }
    };
}
