import { createThemeContract } from "@vanilla-extract/css";

export const vars = createThemeContract({
  color: {
    background: {
      primary: "",
      secondary: "",
      accentPrimary: "",
      focus: "",
      dim: "",
    },
    text: {
      primary: "",
      secondary: "",
      accentPrimary: "",
      placeholder: "",
      input: "",
      dim: "",
      accent: "",
    },
    border: {
      primary: "",
      secondary: "",
    },
    gradient: {
      horizontal: "",
    },
  },
  spacing: {
    xxs: "",
    xs: "",
    sm: "",
    md: "",
    lg: "",
    xl: "",
    xxl: "",
  },
  text: {
    xs: "",
    sm: "",
    md: "",
    lg: "",
    xl: "",
    xxl: "",
  },
  border: {
    solid: "",
    __raw: "",
    radius: {
      sm: "",
      md: "",
      lg: "",
      round: "",
    },
  },
});
