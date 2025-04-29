import { children, Component, JSX } from "solid-js";
import { cva, VariantProps } from "class-variance-authority";
import { twMerge } from "tailwind-merge";

const cardVariants = cva("p-4 z-2", {
  variants: {
      padding: {
          default: "p-2",
          medium: "p-4",
          large: "p-6"
      }
  },
  defaultVariants: {
      padding: "default"
  }
})

export const Card: Component<{ padding: any, children: JSX.Element }> = ({ padding, children, ...props }) => {
  return (
    <div class="border-white/30 border-[1px] bg-black/50 rounded shadow-sm backdrop-blur-lg">
        <div class="absolute bg-[url(./assets/noise.svg)] opacity-5 w-[100%] h-[100%] z-0">
        </div>
        <div class={twMerge(cardVariants( { padding, ...props } ))}>
            {children}
        </div>
    </div>
  );
};
