import * as theme from "@theme/theme.css";
import { vars } from "@theme/contract.css";
import { style, styleVariants, ComplexStyleRule } from "@vanilla-extract/css";

export const button = style({
  width: "100%",
  paddingBottom: "100%",
  backgroundPosition: "center",
  backgroundSize: "contain",
  marginBottom: vars.spacing.xs,
  background: "linear-gradient(90deg, #D478FF 0%, #7E59F6 59.5%, #503DC4 103.86%)",
  border: "none",
  borderRadius: vars.border.radius.round,
  position: "relative",
});

export const buttonIcon = style({
    position: "absolute",
    width: "calc(100% - 3px)",
    height: "calc(100% - 3px)",
    background: vars.color.background.accentPrimary,
    margin: "1.5px",
    top: 0,
    left: 0,
    borderRadius: vars.border.radius.round,
    display: "flex",
    justifyContent: "center",
});