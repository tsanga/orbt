import { style } from "@vanilla-extract/css";
import * as theme from "@theme/theme.css";
import { vars } from "@theme/contract.css";

export const title = style([
  theme.text.xxl,
  {
    fontWeight: 600,
    marginBottom: vars.spacing.md,
  },
]);
