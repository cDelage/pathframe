import { Palette, Shade } from "../../domain/DesignSystemDomain";
import { getClosestToPercentage } from "../../util/DesignSystemUtils";

function PaletteIcon({ palette, size }: { palette: Palette; size: string }) {
  const { shades } = palette;
  const defaultShade: Shade = {
    label: "default",
    color: "#60A5FA",
  };

  const firstShade: string = getClosestToPercentage({
    array: shades,
    defaultValue: defaultShade,
    percentage: 0,
  }).color;
  const secondShade: string = getClosestToPercentage({
    array: shades,
    defaultValue: defaultShade,
    percentage: 0.25,
  }).color;
  const thirdShade: string = getClosestToPercentage({
    array: shades,
    defaultValue: defaultShade,
    percentage: 0.5,
  }).color;
  const fourthShade: string = getClosestToPercentage({
    array: shades,
    defaultValue: defaultShade,
    percentage: 1,
  }).color;

  console.log({
    firstShade,
    secondShade,
    thirdShade,
    fourthShade,
  });

  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 27 27"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M13.3333 26.6667C11.5111 26.6667 9.78889 26.3167 8.16667 25.6167C6.54444 24.9167 5.12778 23.9611 3.91667 22.75C2.70556 21.5389 1.75 20.1222 1.05 18.5C0.35 16.8778 0 15.1556 0 13.3333C0 11.4889 0.361111 9.75555 1.08333 8.13333C1.80556 6.51111 2.78333 5.1 4.01667 3.9C5.25 2.7 6.68889 1.75 8.33333 1.05C9.97778 0.35 11.7333 0 13.6 0C15.3778 0 17.0556 0.305556 18.6333 0.916667C20.2111 1.52778 21.5944 2.37222 22.7833 3.45C23.9722 4.52778 24.9167 5.80556 25.6167 7.28333C26.3167 8.76111 26.6667 10.3556 26.6667 12.0667C26.6667 14.6222 25.8889 16.5833 24.3333 17.95C22.7778 19.3167 20.8889 20 18.6667 20H16.2C16 20 15.8611 20.0556 15.7833 20.1667C15.7056 20.2778 15.6667 20.4 15.6667 20.5333C15.6667 20.8 15.8333 21.1833 16.1667 21.6833C16.5 22.1833 16.6667 22.7556 16.6667 23.4C16.6667 24.5111 16.3611 25.3333 15.75 25.8667C15.1389 26.4 14.3333 26.6667 13.3333 26.6667ZM6 14.6667C6.57778 14.6667 7.05556 14.4778 7.43333 14.1C7.81111 13.7222 8 13.2444 8 12.6667C8 12.0889 7.81111 11.6111 7.43333 11.2333C7.05556 10.8556 6.57778 10.6667 6 10.6667C5.42222 10.6667 4.94444 10.8556 4.56667 11.2333C4.18889 11.6111 4 12.0889 4 12.6667C4 13.2444 4.18889 13.7222 4.56667 14.1C4.94444 14.4778 5.42222 14.6667 6 14.6667ZM10 9.33333C10.5778 9.33333 11.0556 9.14444 11.4333 8.76667C11.8111 8.38889 12 7.91111 12 7.33333C12 6.75556 11.8111 6.27778 11.4333 5.9C11.0556 5.52222 10.5778 5.33333 10 5.33333C9.42222 5.33333 8.94444 5.52222 8.56667 5.9C8.18889 6.27778 8 6.75556 8 7.33333C8 7.91111 8.18889 8.38889 8.56667 8.76667C8.94444 9.14444 9.42222 9.33333 10 9.33333ZM16.6667 9.33333C17.2444 9.33333 17.7222 9.14444 18.1 8.76667C18.4778 8.38889 18.6667 7.91111 18.6667 7.33333C18.6667 6.75556 18.4778 6.27778 18.1 5.9C17.7222 5.52222 17.2444 5.33333 16.6667 5.33333C16.0889 5.33333 15.6111 5.52222 15.2333 5.9C14.8556 6.27778 14.6667 6.75556 14.6667 7.33333C14.6667 7.91111 14.8556 8.38889 15.2333 8.76667C15.6111 9.14444 16.0889 9.33333 16.6667 9.33333ZM20.6667 14.6667C21.2444 14.6667 21.7222 14.4778 22.1 14.1C22.4778 13.7222 22.6667 13.2444 22.6667 12.6667C22.6667 12.0889 22.4778 11.6111 22.1 11.2333C21.7222 10.8556 21.2444 10.6667 20.6667 10.6667C20.0889 10.6667 19.6111 10.8556 19.2333 11.2333C18.8556 11.6111 18.6667 12.0889 18.6667 12.6667C18.6667 13.2444 18.8556 13.7222 19.2333 14.1C19.6111 14.4778 20.0889 14.6667 20.6667 14.6667ZM13.3333 24C13.5333 24 13.6944 23.9444 13.8167 23.8333C13.9389 23.7222 14 23.5778 14 23.4C14 23.0889 13.8333 22.7222 13.5 22.3C13.1667 21.8778 13 21.2444 13 20.4C13 19.4667 13.3222 18.7222 13.9667 18.1667C14.6111 17.6111 15.4 17.3333 16.3333 17.3333H18.6667C20.1333 17.3333 21.3889 16.9056 22.4333 16.05C23.4778 15.1944 24 13.8667 24 12.0667C24 9.37778 22.9722 7.13889 20.9167 5.35C18.8611 3.56111 16.4222 2.66667 13.6 2.66667C10.5778 2.66667 8 3.7 5.86667 5.76667C3.73333 7.83333 2.66667 10.3556 2.66667 13.3333C2.66667 16.2889 3.70556 18.8056 5.78333 20.8833C7.86111 22.9611 10.3778 24 13.3333 24Z"
        fill="#525252"
      />
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M7.43333 14.1C7.05556 14.4778 6.57778 14.6667 6 14.6667C5.42222 14.6667 4.94444 14.4778 4.56667 14.1C4.18889 13.7222 4 13.2444 4 12.6667C4 12.0889 4.18889 11.6111 4.56667 11.2333C4.94444 10.8556 5.42222 10.6667 6 10.6667C6.57778 10.6667 7.05556 10.8556 7.43333 11.2333C7.81111 11.6111 8 12.0889 8 12.6667C8 13.2444 7.81111 13.7222 7.43333 14.1ZM11.4333 8.76667C11.0556 9.14444 10.5778 9.33333 10 9.33333C9.42222 9.33333 8.94444 9.14444 8.56667 8.76667C8.18889 8.38889 8 7.91111 8 7.33333C8 6.75556 8.18889 6.27778 8.56667 5.9C8.94444 5.52222 9.42222 5.33333 10 5.33333C10.5778 5.33333 11.0556 5.52222 11.4333 5.9C11.8111 6.27778 12 6.75556 12 7.33333C12 7.91111 11.8111 8.38889 11.4333 8.76667ZM18.1 8.76667C17.7222 9.14444 17.2444 9.33333 16.6667 9.33333C16.0889 9.33333 15.6111 9.14444 15.2333 8.76667C14.8556 8.38889 14.6667 7.91111 14.6667 7.33333C14.6667 6.75556 14.8556 6.27778 15.2333 5.9C15.6111 5.52222 16.0889 5.33333 16.6667 5.33333C17.2444 5.33333 17.7222 5.52222 18.1 5.9C18.4778 6.27778 18.6667 6.75556 18.6667 7.33333C18.6667 7.91111 18.4778 8.38889 18.1 8.76667ZM22.1 14.1C21.7222 14.4778 21.2444 14.6667 20.6667 14.6667C20.0889 14.6667 19.6111 14.4778 19.2333 14.1C18.8556 13.7222 18.6667 13.2444 18.6667 12.6667C18.6667 12.0889 18.8556 11.6111 19.2333 11.2333C19.6111 10.8556 20.0889 10.6667 20.6667 10.6667C21.2444 10.6667 21.7222 10.8556 22.1 11.2333C22.4778 11.6111 22.6667 12.0889 22.6667 12.6667C22.6667 13.2444 22.4778 13.7222 22.1 14.1Z"
        fill={firstShade}
      />
      <path
        d="M11.4333 8.76667C11.0556 9.14444 10.5778 9.33333 10 9.33333C9.42222 9.33333 8.94444 9.14444 8.56667 8.76667C8.18889 8.38889 8 7.91111 8 7.33333C8 6.75556 8.18889 6.27778 8.56667 5.9C8.94444 5.52222 9.42222 5.33333 10 5.33333C10.5778 5.33333 11.0556 5.52222 11.4333 5.9C11.8111 6.27778 12 6.75556 12 7.33333C12 7.91111 11.8111 8.38889 11.4333 8.76667Z"
        fill={firstShade}
      />
      <path
        d="M11.4333 8.76668C11.0556 9.14446 10.5778 9.33334 10 9.33334C9.42222 9.33334 8.94444 9.14446 8.56667 8.76668C8.18889 8.3889 8 7.91112 8 7.33334C8 6.75557 8.18889 6.27779 8.56667 5.90001C8.94444 5.52223 9.42222 5.33334 10 5.33334C10.5778 5.33334 11.0556 5.52223 11.4333 5.90001C11.8111 6.27779 12 6.75557 12 7.33334C12 7.91112 11.8111 8.3889 11.4333 8.76668Z"
        fill={secondShade}
      />
      <path
        d="M18.1003 8.76668C17.7225 9.14446 17.2448 9.33334 16.667 9.33334C16.0892 9.33334 15.6114 9.14446 15.2337 8.76668C14.8559 8.3889 14.667 7.91112 14.667 7.33334C14.667 6.75557 14.8559 6.27779 15.2337 5.90001C15.6114 5.52223 16.0892 5.33334 16.667 5.33334C17.2448 5.33334 17.7225 5.52223 18.1003 5.90001C18.4781 6.27779 18.667 6.75557 18.667 7.33334C18.667 7.91112 18.4781 8.3889 18.1003 8.76668Z"
        fill={thirdShade}
      />
      <path
        d="M22.1003 14.1C21.7225 14.4778 21.2448 14.6667 20.667 14.6667C20.0892 14.6667 19.6114 14.4778 19.2337 14.1C18.8559 13.7222 18.667 13.2445 18.667 12.6667C18.667 12.0889 18.8559 11.6111 19.2337 11.2334C19.6114 10.8556 20.0892 10.6667 20.667 10.6667C21.2448 10.6667 21.7225 10.8556 22.1003 11.2334C22.4781 11.6111 22.667 12.0889 22.667 12.6667C22.667 13.2445 22.4781 13.7222 22.1003 14.1Z"
        fill={fourthShade}
      />
    </svg>
  );
}

export default PaletteIcon;
