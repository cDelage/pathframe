import styled from "styled-components";

export const TextArea = styled.textarea`
  border: none;
  resize: none;
  border-radius: var(--radius);
  box-shadow: var(--shadow-solid);
  padding: var(--space-4);
  font-family: inherit;
  box-sizing: border-box;
  background-color: var(--bg-default);
  color: var(--text-main);
  border: var(--border-main) 1px solid;
`;
