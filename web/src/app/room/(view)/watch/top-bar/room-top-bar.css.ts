import { style } from "@vanilla-extract/css";
import { vars } from "@theme/contract.css";

export const nav = style({
  padding: vars.spacing.lg,
  borderBottom: vars.border.solid,
  borderColor: vars.color.border.primary,
  background: vars.color.background.accentPrimary,
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
});
