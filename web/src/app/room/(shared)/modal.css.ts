import { style } from "@vanilla-extract/css";
import { vars } from "@theme/contract.css";

export const backdrop = style({
  background: "black",
  opacity: 0.65,
  position: "absolute",
  width: "100%",
  height: "100%",
});

export const container = style({
  position: "fixed",
  width: "100vw",
  height: "100vh",
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
  zIndex: 999,
});

export const modal = style({
  position: "relative",
  background: vars.color.background.accentPrimary,
  padding: vars.spacing.xl,
  borderRadius: vars.border.radius.md,
  minWidth: "350px",
});

export const closeButton = style({
  position: "absolute",
  top: vars.spacing.sm,
  right: vars.spacing.sm,
  color: vars.color.text.accent,
  transition: "opacity 0.1s",
  cursor: "pointer",
  ":hover": {
    opacity: 0.5,
  },
});
