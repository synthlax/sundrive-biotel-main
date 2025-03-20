import { createEffect, Suspense } from "solid-js";

export default function Contact() {
    return (
        <div class="flex flex-col w-[100%] h-[100vh] bg-linear-to-b from-[#100400] to-50% to-[#000000] top-[10%] px-4 py-4 items-center ">
            <div class="flex flex-col w-[60%] h-[90%] border-2 rounded-[8px]">
                <h1 class="text-4xl text-white px-[2%] font-bold py-4 bg-[#1e1e1e]/50 border-b-1">
                    Contact & Acknowledgements
                </h1>

                <ul class="flex mt-4 text-white px-[2%] list-none flex-col">
                    <li class="">
                        Company Email:{" "}
                        <span class="underline text-orange-500 after:content-['_↗']">
                            corporate@sundrive.company
                        </span>
                    </li>
                    <li class="">Operations Contact: Barnett A.</li>
                    <li class="">Active Maintainer: Angel A.</li>
                    <li class="">Contributors:</li>
                </ul>
                <form class="px-[2%] flex-row mt-2.5 gap-4 scheme-dark-only">
                    <div class="flex flex-col">
                        <label class="text-white">Full Name</label>
                        <input placeholder="e.g John J. Doe" required class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"></input>
                    </div>
                    <div class="flex flex-col">
                        <label class="text-white">Subject</label>
                        <input placeholder="e.g Contribution Request" minLength={1} required class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"></input>
                    </div>
                    <div class="flex flex-col">
                        <label class="text-white">Email</label>
                        <input placeholder="example@domain.com" type="email" required class="w-[100%] h-[5vh] bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 invalid:border-red-400"></input>
                    </div>
                    <div class="">
                        <label class="text-white">Body</label>
                        <textarea placeholder="Input here..." minLength={1} required class="w-[100%] h-32 bg-[#5B5B5B]/50 rounded-[4px] border-1 border-black/50 hover:border-white/50 duration-300 text-[#a8a8a8] focus:text-white focus:border-white/50 outline-0 px-2 text-pretty break-normal py-2 resize-none invalid:border-red-400"></textarea>
                    </div>
                    <button class=" mt-3 w-[100%] text-white bg-[#FF4400] px-4 py-2 rounded-[4px] hover:text-[#cccccc] hover:bg-[#ff5f25] hover:shadow-[0px_0px_100px_rgba(255,95,37,.5)] duration-300 hover:cursor-pointer">Sumbit</button>
                </form>

            </div>
        </div>
    );
}
