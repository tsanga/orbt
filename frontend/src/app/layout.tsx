import { Inter } from "@next/font/google";
import { background } from "@theme";

const inter = Inter();

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={inter.className}>
      <body className={background.primary}>{children}</body>
    </html>
  );
}
