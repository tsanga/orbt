import { style } from "@vanilla-extract/css";
import * as theme from "@theme/theme.css";
import { vars } from "@theme/contract.css";

export const container = style({
  borderLeft: vars.border.solid,
  borderColor: vars.color.border.primary,
  height: `calc(100vh - (${vars.spacing.sm} * 2))`,
  background: vars.color.background.accentPrimary,
  display: "flex",
  flexDirection: "column",
  width: "60px",
  alignItems: "center",
  padding: vars.spacing.sm,
});

export const participantSkeleton = style([
  theme.animate.pulse,
  theme.background.dim,
  {
    width: "100%",
    paddingBottom: "100%",
    borderRadius: vars.border.radius.round,
    marginBottom: vars.spacing.xs,
    border: vars.border.solid,
    borderColor: "transparent",
  },
]);
