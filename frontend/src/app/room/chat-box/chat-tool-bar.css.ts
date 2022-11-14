import { style } from "@vanilla-extract/css";
import { vars } from "@theme";

export const toolbar = style({
  display: "flex",
  width: "100%",
  alignItems: "center",
  justifyContent: "flex-end",
});

export const cog = style({
  marginRight: vars.spacing.xs,
});

export const minimizeChatButton = style({
  transform: "scaleX(-1)",
  display: "flex",
  marginRight: "auto",
});
