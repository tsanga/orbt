import { style, globalStyle } from "@vanilla-extract/css";
import { vars, theme } from "@theme";

export const toolbar = style({
  display: "flex",
  width: "100%",
  alignItems: "center",
  justifyContent: "flex-end",
});

export const cog = style({
  marginRight: vars.spacing.xs,
  transition: "transform 0.5s",
  ":hover": {
    transform: "rotate(90deg)",
  },
});

export const minimizeChatButton = style({
  display: "flex",
  marginRight: "auto",
  transition: "transform 0.5s",
  ":hover": {
    transform: "translateX(1px)",
  },
});

export const minimizeChatButtonArrow = style({
  transition: "transform 0.5s",
  transform: "translateX(1px)",
});

globalStyle(`${minimizeChatButton}:hover ${minimizeChatButtonArrow}`, {
  transform: "translateX(2.5px)",
});
