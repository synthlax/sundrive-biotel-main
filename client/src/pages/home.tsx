import { createSignal } from 'solid-js';
import { BACKGROUND_SVG } from '../components/home-bg';
import { A } from '@solidjs/router';

export default function Home() {

  return (
    <div class="flex-col w-[100%] h-[300vh] bg-linear-to-b from-[#100400] to-50% to-[#000000]">
      <div class="w-full h-screen order-1 items-center relative">
        <BACKGROUND_SVG></BACKGROUND_SVG>
        <div class="absolute flex flex-col z-[1] w-[60%] h-[80%] left-[20%] bottom-[10%] items-center gap-y-10">
          <div class="relative flex flex-col items-center gap-y-10">
            <svg
              width={200}
              height={131}
              viewBox="0 0 200 131"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              class="drop-shadow-[0px_0px_200px_rgba(255,68,0,0.5)]"
              preserveAspectRatio="xMidYMid meet"
            >
              <path
                d="M170.979 0C178.863 0 185.059 1.54627 189.569 4.6388C194.195 7.73135 197.168 11.8754 198.488 17.0708C199.809 22.2663 199.511 28.0184 197.594 34.3272C195.603 40.8838 192.242 46.6179 187.513 51.5294L154.265 51.4C157.099 50.8856 159.699 49.8335 162.064 48.2436C165.598 45.7696 168.116 42.0585 169.619 37.1105C171.085 32.2861 170.783 28.6988 168.715 26.3484C166.683 23.8744 163.407 22.6374 158.886 22.6374H138.364L129.657 51.3043L102.733 51.1995L111.49 22.3692L70.7387 23.2006C70.7387 23.2006 57.7192 25.4781 57.7192 38.3836C57.7192 51.2892 70.8229 51.2892 70.8229 51.2892H102.584L94.5985 77.915L122.568 77.7686L112.326 108.363H135.108C140.441 108.363 145.122 106.693 149.151 103.353C153.18 100.013 155.927 95.9306 157.392 91.1062C158.407 87.7663 158.613 84.7974 158.011 82.1997C157.591 80.3909 156.809 78.852 155.664 77.583L185.511 77.4204C186.217 82.049 185.699 87.2294 183.958 92.9617C181.779 100.136 178.125 106.631 172.997 112.445C168.022 118.135 161.898 122.65 154.622 125.99C147.463 129.33 139.536 131 130.84 131L76.4166 131L76.4117 131H76.3697L0 130.241L6.98351 107.466L84.6145 106.822L94.4634 77.9405L17.5009 77.4669L43.079 0.0465497C72.9052 0.0465497 113.37 0.0114269 118.282 0.00706321L118.285 0H170.979Z"
                fill="url(#paint0_linear_3_445)"
              />
              <defs>
                <linearGradient
                  id="paint0_linear_3_445"
                  x1="41.8522"
                  y1="-0.00000643426"
                  x2="199.748"
                  y2="130.453"
                  gradientUnits="userSpaceOnUse"
                >
                  <stop stop-color="#953100" />
                  <stop offset={1} stop-color="#BA8365" />
                </linearGradient>
              </defs>
            </svg>
            <div class="flex flex-col order-2 items-center gap-2.5">
              <p class="text-4xl font-bold text-center capitalize text-[#f5ede3]">
                <span class="font-bold text-center capitalize text-[#f5ede3]">
                  Power Your Data:{" "}
                </span>
                <br />
                <span class="font-bold text-center capitalize text-[#f5ede3]">
                  Bioinformatics for Discovery
                </span>
              </p>
              <p class=" text-2xl text-center capitalize text-[#f5ede3] w-[75%] break-keep">
                Transform raw biological data into comprehensive insights through automation, analysis, and
                visualization.
              </p>
            </div>
          </div>

          <div class="flex justify-start items-center gap-4 z-[1]">
            <button class="relative overflow-hidden gap-2.5 px-4 py-2 rounded bg-[#212121]/25 border border-[#999] backdrop-blur-[25px] text-white hover:bg-white hover:text-black duration-300">
              <A href="/docs">
                Self-Host Biotel
              </A>
            </button>
            <button class="relative overflow-hidden gap-2.5 px-4 py-2 rounded bg-[#ff4400] border border-black/30 text-white hover:bg-[#ff5f25] hover:text-[#cccccc] hover:shadow-[0px_0px_100px_rgba(255,95,37,.5)] duration-300">
              <A href="/login">
                Create Account
              </A>
            </button>
          </div>

          <div class="flex justify-center items-center relative top-4">
            <svg
              width={22}
              height={22}
              viewBox="0 0 22 22"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              class=" w-[22px] h-[22px] relative"
              preserveAspectRatio="xMidYMid meet"
            >
              <path
                d="M12.375 1.375H9.625C8.1668 1.37659 6.76878 1.95657 5.73767 2.98767C4.70657 4.01878 4.12659 5.4168 4.125 6.875V15.125C4.12659 16.5832 4.70657 17.9812 5.73767 19.0123C6.76878 20.0434 8.1668 20.6234 9.625 20.625H12.375C13.8332 20.6234 15.2312 20.0434 16.2623 19.0123C17.2934 17.9812 17.8734 16.5832 17.875 15.125V6.875C17.8734 5.4168 17.2934 4.01878 16.2623 2.98767C15.2312 1.95657 13.8332 1.37659 12.375 1.375ZM16.5 15.125C16.4989 16.2187 16.0639 17.2672 15.2906 18.0406C14.5172 18.8139 13.4687 19.2489 12.375 19.25H9.625C8.53133 19.2489 7.48278 18.8139 6.70944 18.0406C5.9361 17.2672 5.50114 16.2187 5.5 15.125V6.875C5.50114 5.78133 5.9361 4.73278 6.70944 3.95944C7.48278 3.1861 8.53133 2.75114 9.625 2.75H12.375C13.4687 2.75114 14.5172 3.1861 15.2906 3.95944C16.0639 4.73278 16.4989 5.78133 16.5 6.875V15.125ZM11.6875 7.15945V14.8405L12.5761 13.9511C12.7051 13.8221 12.8801 13.7496 13.0625 13.7496C13.2449 13.7496 13.4199 13.8221 13.5489 13.9511C13.6779 14.0801 13.7504 14.2551 13.7504 14.4375C13.7504 14.6199 13.6779 14.7949 13.5489 14.9239L11.4864 16.9864C11.4226 17.0503 11.3467 17.101 11.2633 17.1356C11.1798 17.1702 11.0903 17.188 11 17.188C10.9097 17.188 10.8202 17.1702 10.7367 17.1356C10.6533 17.101 10.5774 17.0503 10.5136 16.9864L8.45109 14.9239C8.32209 14.7949 8.24962 14.6199 8.24962 14.4375C8.24962 14.2551 8.32209 14.0801 8.45109 13.9511C8.5801 13.8221 8.75506 13.7496 8.9375 13.7496C9.11994 13.7496 9.2949 13.8221 9.42391 13.9511L10.3125 14.8405V7.15945L9.42391 8.04891C9.2949 8.17791 9.11994 8.25038 8.9375 8.25038C8.75506 8.25038 8.5801 8.17791 8.45109 8.04891C8.32209 7.9199 8.24962 7.74494 8.24962 7.5625C8.24962 7.38006 8.32209 7.2051 8.45109 7.07609L10.5136 5.01359C10.5774 4.94967 10.6533 4.89896 10.7367 4.86437C10.8202 4.82977 10.9097 4.81196 11 4.81196C11.0903 4.81196 11.1798 4.82977 11.2633 4.86437C11.3467 4.89896 11.4226 4.94967 11.4864 5.01359L13.5489 7.07609C13.6128 7.13997 13.6635 7.2158 13.698 7.29926C13.7326 7.38272 13.7504 7.47217 13.7504 7.5625C13.7504 7.65283 13.7326 7.74228 13.698 7.82574C13.6635 7.9092 13.6128 7.98503 13.5489 8.04891C13.485 8.11278 13.4092 8.16345 13.3257 8.19802C13.2423 8.23259 13.1528 8.25038 13.0625 8.25038C12.9722 8.25038 12.8827 8.23259 12.7993 8.19802C12.7158 8.16345 12.64 8.11278 12.5761 8.04891L11.6875 7.15945Z"
                fill="white"
              />
            </svg>
            <p class="text-lg text-center text-white">Scroll For Fun</p>
          </div>

        </div>
      </div>

    </div>
  );
}
