<script lang="ts">
  import { Avatar, AvatarFallback, AvatarImage } from '$lib/components/ui/avatar/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { MessageCircle, Repeat2, Heart, Bookmark, Upload } from '@lucide/svelte';
  import type { Tuna } from './types';

  let { tuna }: { tuna: Tuna } = $props();
</script>

<div class="flex items-start gap-3 px-4 py-3">
  <Avatar class="h-10 w-10">
    <AvatarImage src={tuna.user.avatar ?? '/placeholder.svg'} alt={tuna.user.name} />
    <AvatarFallback>{tuna.user.name[0]}</AvatarFallback>
  </Avatar>
  <div class="grid flex-1 gap-1">
    <div class="flex items-center gap-1">
      <a class="font-bold text-white hover:underline dark:text-black" href="/profile/{tuna.user.handle.slice(1)}">
        {tuna.user.name}
      </a> <span class="text-sm text-[#71767b] dark:text-gray-500">{tuna.user.handle}</span>
      <span class="text-sm text-[#71767b] dark:text-gray-500">· {tuna.timestamp}</span>
    </div>
    <p class="text-base leading-relaxed text-white dark:text-black">{tuna.content}</p>

    {#if tuna.image}
      <div class="mt-2 overflow-hidden rounded-xl">
        <img src={tuna.image} alt="Tuna" class="h-auto w-full object-cover" width="600" height="400" />
      </div>
    {/if}
    <div class="mt-2 flex items-center justify-between text-sm text-[#71767b]">
      {#each [{ icon: MessageCircle, count: tuna.comments, label: '댓글', hover: 'text-[#1d9bf0]' }, { icon: Repeat2, count: tuna.retunas, label: '리트윗', hover: 'text-green-500' }, { icon: Heart, count: tuna.likes, label: '좋아요', hover: 'text-red-500' }] as { icon: Icon, count, label, hover }}
        <Button
          variant="ghost"
          size="sm"
          class="flex items-center gap-1 {hover} rounded-full dark:text-gray-500 dark:hover:{hover.replace(
            'text-',
            'text-'
          )}-700"
        >
          <Icon class="h-4 w-4" /> <span>{count}</span> <span class="sr-only">{label}</span>
        </Button>
      {/each}
      <div class="flex gap-1">
        {#each [Bookmark, Upload] as Icon}
          <Button
            variant="ghost"
            size="sm"
            class="flex items-center gap-1 rounded-full text-[#71767b] hover:text-[#1d9bf0] dark:text-gray-500 dark:hover:text-blue-700"
          >
            <Icon class="h-4 w-4" /> <span class="sr-only">action</span>
          </Button>
        {/each}
      </div>
    </div>
  </div>
</div>
