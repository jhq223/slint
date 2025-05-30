---
<!-- Copyright © SixtyFPS GmbH <info@slint.dev> ; SPDX-License-Identifier: MIT -->
title: Overview
description: Widgets Overview.
---
Slint provides a series of built-in widgets that can be imported from `"std-widgets.slint"`.

The widget appearance depends on the selected style.
See [Selecting a Widget Style](../../advanced/style.md#selecting-a-widget-style) for details how to select the style. If no style is selected, `native` is the default. If `native` isn't available, `fluent` is the default.


All widgets support all [properties common to builtin elements](../builtins/elements.md#common-properties).

## `Palette`

Use `Palette` to create custom widgets that match the colors of
the selected style e.g. fluent, cupertino, material, or qt.

### Properties

-   **`background`** (_out_ _brush_): Defines the default background brush. Use this if none of the more specialized background brushes apply.
-   **`foreground`** (_out_ _brush_): Defines the foreground brush that is used for content that is displayed on `background` brush.
-   **`alternate-background`** (_out_ _brush_): Defines an alternate background brush that is used for example for text input controls or panels like a side bar.
-   **`alternate-foreground`** (_out_ _brush_): Defines the foreground brush that is used for content that is displayed on `alternate-background` brush.
-   **`control-background`** (_out_ _brush_): Defines the default background brush for controls, such as push buttons, combo boxes, etc.
-   **`control-foreground`** (_out_ _brush_): Defines the foreground brush that is used for content that is displayed on `control-background` brush.
-   **`accent-background`** (_out_ _brush_): Defines the background brush for highlighted controls such as primary buttons.
-   **`accent-foreground`** (_out_ _brush_): Defines the foreground brush that is used for content that is displayed on `accent-background` brush.
-   **`selection-background`** (_out_ _brush_): Defines the background brush that is used to highlight a selection such as a text selection.
-   **`selection-foreground`** (_out_ _brush_):  Defines the foreground brush that is used for content that is displayed on `selection-background` brush.
-   **`border`** (_out_ _brush_): Defines the brush that is used for borders such as separators and widget borders.
-   **`color-scheme`** (_in_ _out_ _enum [`ColorScheme`](enums.md#colorscheme)_): Read this property to determine the color scheme used by the palette.
    Set this property to force a dark or light color scheme. All styles except for the Qt style support setting a dark or light color scheme.

### Example

```slint
import { Palette, HorizontalBox } from "std-widgets.slint";

export component MyCustomWidget {
    in property <string> text <=> label.text;

    Rectangle {
        background: Palette.control-background;

        HorizontalBox {
            label := Text {
                color: Palette.control-foreground;
            }
        }
    }
}
```

## `LayoutSpec`

Use `LayoutSpec` to create custom widgets that match the layout settings of
the selected style e.g. fluent, cupertino, material, or qt.

### Properties

-   **`layout-spacing`** (_out_ _length_): Defines the default layout spacing. This spacing is also used by `VerticalBox`, `HorizontalBox` and `GridBox`.
-   **`layout-padding`** (_out_ _length_): Defines the default layout padding. This padding is also used by `VerticalBox`, `HorizontalBox` and `GridBox`.

