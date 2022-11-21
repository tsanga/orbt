import * as theme from "@theme/theme.css";
import { vars } from "@theme/contract.css";
import { style, styleVariants, ComplexStyleRule } from "@vanilla-extract/css";
import { Status } from "@domain/models";

const base = style({
  borderRadius: vars.border.radius.round,
  width: "100%",
  paddingBottom: "100%",
  backgroundPosition: "center",
  backgroundSize: "contain",
  marginBottom: vars.spacing.xs,
  border: vars.border.solid,
  borderColor: vars.color.border.primary,
  backgroundColor: vars.color.background.dim,
  transition: "all 1.5s",
});

const statusMap: Record<Status, ComplexStyleRule> = {
  [Status.CONNECTED]: [base, {}],
  [Status.AWAY]: [base, theme.animate.pulse, {}],
  [Status.CONNECTING]: [base, theme.animate.pulseLow],
  [Status.DISCONNECTED]: [base, { opacity: 0.2 }],
};

export const participant = styleVariants(statusMap);
