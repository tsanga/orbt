import { style } from "@vanilla-extract/css";
import { vars } from "@theme";

export const container = style({
  borderLeft: vars.border.solid,
  borderColor: vars.color.border.primary,
  height: `calc(100vh)`,
  background: vars.color.background.accentPrimary,
  display: "flex",
  flexDirection: "column",
  minWidth: "100px",
});
