import { Inter } from "@next/font/google";
import { cookies } from "next/headers";
import { darkThemeClass, lightThemeClass, theme } from "@theme";
import "reset-css/reset.css";

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
        {children}
      </body>
    </html>
  );
}
