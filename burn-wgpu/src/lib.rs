#[macro_use]
extern crate derive_new;

mod ops;

pub(crate) mod context;
pub(crate) mod element;
pub(crate) mod kernel;
pub(crate) mod pool;
pub(crate) mod tensor;

mod device;
pub use device::*;

mod backend;
pub use backend::*;

mod graphics;
pub use graphics::*;

#[cfg(test)]
mod tests {
    use super::*;

    type TestBackend = WGPUBackend<Vulkan, f32, i64>;
    type TestTensor<const D: usize> = burn_tensor::Tensor<TestBackend, D>;
    // type TestTensorInt<const D: usize> = burn_tensor::Tensor<TestBackend, D, burn_tensor::Int>;

    burn_tensor::testgen_add!();
    burn_tensor::testgen_sub!();
    burn_tensor::testgen_div!();
    burn_tensor::testgen_mul!();
    burn_tensor::testgen_neg!();
    burn_tensor::testgen_powf!();
    burn_tensor::testgen_exp!();
    burn_tensor::testgen_log!();
    burn_tensor::testgen_relu!();
    burn_tensor::testgen_matmul!();

    type TestADBackend = burn_autodiff::ADBackendDecorator<TestBackend>;
    type TestADTensor<const D: usize, K> = burn_tensor::Tensor<TestADBackend, D, K>;

    burn_autodiff::testgen_ad_add!();
    burn_autodiff::testgen_ad_sub!();

    // Once all operations will be implemented.
    // burn_tensor::testgen_all!();
    // burn_autodiff::testgen_all!();
}