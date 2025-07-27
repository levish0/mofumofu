<script lang="ts">
  import { onMount } from 'svelte';
  import gsap from 'gsap';
  import { Button } from '$lib/components/ui/button/index.js';
  import { Avatar, AvatarFallback, AvatarImage } from '$lib/components/ui/avatar/index.js';
  import { Home, Search, Bell, Mail, Bookmark, User, Ellipsis, Crown, Cog, SunIcon, MoonIcon } from '@lucide/svelte';
  import { toggleMode } from 'mode-watcher';

  let navItems: HTMLElement[] = [];

  onMount(() => {
    navItems.forEach((element) => {
      const icon = element.querySelector('svg');
      const label = element.querySelector('span');

      const tl = gsap.timeline({ paused: true });

      tl.to(icon, {
        scale: 1.15,
        rotation: 5,
        duration: 0.4,
        ease: 'elastic.out(1.2,0.8)'
      }).to(
        label,
        {
          x: 6,
          duration: 0.2,
          ease: 'power2.out'
        },
        0
      );

      element.addEventListener('mouseenter', () => tl.play());
      element.addEventListener('mouseleave', () => tl.reverse());
    });
  });
</script>

<div class="sticky top-0 hidden h-screen w-[275px] flex-col justify-between px-4 py-2 lg:flex">
  <div class="flex flex-col items-start gap-2">
    <a
      class="rounded-full p-2 text-4xl transition-colors hover:bg-gray-800 dark:text-black dark:hover:bg-gray-100"
      href="/"
    >
      Tuna
    </a>

    <nav class="grid gap-1 text-lg font-medium">
      {#each [{ icon: Home, label: 'Home' }, { icon: Search, label: 'Explore' }, { icon: Bell, label: 'Notifications' }, { icon: Mail, label: 'Messages' }, { icon: Bookmark, label: 'Bookmarks' }, { icon: Crown, label: 'Premium' }, { icon: User, label: 'Profile' }, { icon: Cog, label: 'Settings' }] as { icon: Icon, label }, i}
        <a
          bind:this={navItems[i]}
          class="nav-item flex items-center gap-4 rounded-full px-4 py-2 transition-colors"
          href="/"
        >
          <Icon class="h-5 w-5" />
          <span>{label}</span>
        </a>
      {/each}

      <button
        onclick={toggleMode}
        class="flex w-full items-center gap-4 rounded-full px-4 py-2 text-left transition-colors"
      >
        <SunIcon class="block h-7 w-7 !transition-all dark:hidden" />
        <MoonIcon class="hidden h-7 w-7 !transition-all dark:block" />
        <span>Toggle theme</span>
      </button>
    </nav>

    <Button
      class="mt-4 h-12 w-full rounded-full bg-black text-base font-bold text-white hover:bg-zinc-800  dark:bg-white  dark:text-black dark:hover:bg-gray-300"
    >
      Post
    </Button>
  </div>

  <div class="flex cursor-pointer items-center gap-3 rounded-full p-2 transition-colors">
    <Avatar class="h-10 w-10">
      <AvatarImage src="/placeholder.svg?height=40&width=40" alt="@levish" />
      <AvatarFallback>LL</AvatarFallback>
    </Avatar>
    <div class="grid flex-1 leading-none">
      <div class="font-semibold text-white">levish</div>
      <div class="text-sm text-[#71767b]">@levi_sh00</div>
    </div>
    <Ellipsis class="h-5 w-5 text-white" />
  </div>
</div>
