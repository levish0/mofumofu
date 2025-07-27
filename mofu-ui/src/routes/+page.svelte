<script lang="ts">
  import type { Tuna } from '$lib/components/types';
  import { LeftSidebar, RightSidebar } from '$lib/components/sidebars';
  import Composer from '$lib/components/Composer.svelte';
  import TunaCard from '$lib/components/TunaCard.svelte';

  const tunas: Tuna[] = [
    {
      id: '1',
      user: { name: 'すぴおかか', handle: '@Suyasuyabi427', avatar: '/placeholder.svg?height=40&width=40' },
      content: '',
      timestamp: '17h',
      image:
        'https://media.discordapp.net/attachments/797979019425349682/1395538968849940620/KakaoTalk_20250608_170857003_12.jpg?ex=687ec4f2&is=687d7372&hm=da970e76f95d2a541500f0dfddada8e67cb8a04ed81c556841e6a35111de9137&=&format=webp&width=608&height=525',
      likes: 4900,
      retunas: 702,
      comments: 8
    },
    {
      id: '2',
      user: {
        name: '3 random letters every 10 mins',
        handle: '@3letter_',
        avatar: '/placeholder.svg?height=40&width=40'
      },
      content: 'Automated',
      timestamp: '6h',
      likes: 0,
      retunas: 0,
      comments: 0
    },
    {
      id: '3',
      user: { name: 'John Doe', handle: '@johndoe', avatar: '/placeholder.svg?height=40&width=40' },
      content: '텍스트 기반 트위터를 막 시작했습니다! 방해 없이 생각을 공유하게 되어 기쁩니다. #TextOnly #Minimalist',
      timestamp: '2시간 전',
      likes: 15,
      retunas: 3,
      comments: 2
    }
  ];
</script>

<svelte:head>
  <title>TextTuna • Home</title>
</svelte:head>

<div class="flex min-h-screen w-full justify-center bg-white text-black dark:bg-black dark:text-white">
  <div class="grid w-full max-w-7xl lg:grid-cols-[275px_1fr_350px]">
    <LeftSidebar />

    <main class="flex min-h-screen flex-col">
      <div class="flex justify-around text-lg font-bold">
        <a href="/" class="relative flex-1 py-4 text-center transition-colors hover:bg-gray-800 dark:hover:bg-gray-100">
          For you
          <span class="absolute bottom-0 left-1/2 h-1 w-14 -translate-x-1/2 rounded-full bg-[#1d9bf0]"></span>
        </a>
        <a
          href="/"
          class="flex-1 py-4 text-center text-[#71767b] transition-colors hover:bg-gray-800 dark:text-gray-600 dark:hover:bg-gray-100"
        >
          Following
        </a>
      </div>

      <div class="py-4">
        <Composer />
      </div>

      <div class="flex-1 overflow-y-auto">
        {#each tunas as tuna (tuna.id)}
          <TunaCard {tuna} />
        {/each}
      </div>
    </main>

    <RightSidebar />
  </div>
</div>
