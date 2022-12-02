import { style } from "@vanilla-extract/css";
import * as theme from "@theme/theme.css";

export const title = style([
  theme.text.lg,
  {
    fontWeight: 500,
  },
]);
