#pragma once

#include <iostream>
#include <vector>

#include "tensor.h"
#include "parameter.h"

namespace L2
{
template <class T>
class Layer
{
protected:
    Tensor<T> cached;

public:
    std::vector<Parameter<T>> parameters;

    Parameter<T> build_param(Tensor<T> tensor);
    void update(); // IMPLEMENT after optimizer

    // override in all derived classes
    virtual Tensor<T> forward(Tensor<T> tensor)
    {
        return tensor;
    }

    // override in all derived classes
    virtual Tensor<T> backward(Tensor<T> derivative)
    {
        return derivative;
    }
};
} // namespace L2