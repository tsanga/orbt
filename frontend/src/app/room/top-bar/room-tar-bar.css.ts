import { style } from "@vanilla-extract/css";
import { vars } from "@theme";

export const nav = style({
  padding: vars.spacing.xl,
  borderBottom: vars.border.solid,
  borderColor: vars.color.border.primary,
  background: vars.color.background.accentPrimary,
  display: "flex",
  alignItems: "center",
});
