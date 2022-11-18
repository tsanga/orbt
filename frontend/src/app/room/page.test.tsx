import React from "react";
import { render, screen } from "@testing-library/react";
import RoomPage from "./page";

describe("Room Page", () => {
  it("renders chat box", () => {
    render(<RoomPage />);

    const chatBox = screen.getByTestId("chat-box");

    expect(chatBox).toBeInTheDocument();
  });
});
