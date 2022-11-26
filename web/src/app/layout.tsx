import { Inter } from "@next/font/google";
import { cookies } from "next/headers";
import * as theme from "@theme/theme.css";
import { lightThemeClass } from "@theme/light-theme.css";
import { darkThemeClass } from "@theme/dark-theme.css";
import "reset-css/reset.css";
import RelayEnvironmentProviderWrapper from "./util/relay-environment-provider-wrapper";
import AppContextWrapper from "./context";

const inter = Inter();
export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const nextCookies = cookies();
  const themeCookie = nextCookies.get("theme");

  const themeClass =
    themeCookie?.value === "light" ? lightThemeClass : darkThemeClass;

  return (
    <html lang="en" className={inter.className}>
      <body
        className={`${themeClass} ${theme.background.primary} ${theme.textColor.primary}`}
      >
        <RelayEnvironmentProviderWrapper>
          <AppContextWrapper>{children}</AppContextWrapper>
        </RelayEnvironmentProviderWrapper>
      </body>
    </html>
  );
}
