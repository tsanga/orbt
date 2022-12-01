import RoomContextWrapper from "./context";

export default function RoomLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return <RoomContextWrapper>{children}</RoomContextWrapper>;
}
