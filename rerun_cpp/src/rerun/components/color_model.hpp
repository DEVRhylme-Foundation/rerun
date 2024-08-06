// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/components/color_model.fbs".

#pragma once

#include "../result.hpp"

#include <cstdint>
#include <memory>

namespace arrow {
    /// \private
    template <typename T>
    class NumericBuilder;

    class Array;
    class DataType;
    class UInt8Type;
    using UInt8Builder = NumericBuilder<UInt8Type>;
} // namespace arrow

namespace rerun::components {
    /// **Component**: Specified what color components are present in an `archetypes::Image`.
    ///
    /// This combined with `components::ChannelDatatype` determines the pixel format of an image.
    enum class ColorModel : uint8_t {

        /// Grayscale luminance intencity/brightness/value, sometimes called `Y`
        L = 1,

        /// Red, Green, Blue
        RGB = 2,

        /// Red, Green, Blue, Alpha
        RGBA = 3,
    };
} // namespace rerun::components

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<components::ColorModel> {
        static constexpr const char Name[] = "rerun.components.ColorModel";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::components::ColorModel` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::ColorModel* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::UInt8Builder* builder, const components::ColorModel* elements,
            size_t num_elements
        );
    };
} // namespace rerun
