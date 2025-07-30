<script lang="ts">
	import PostCard from '$lib/components/post/PostCard.svelte';

	interface Author {
		name: string;
		avatar: string;
	}

	interface Card {
		id: number;
		image?: string;
		title: string;
		summary: string;
		date: string;
		comments: number;
		views: string;
		author: Author;
		likes: number;
	}

	let cards: Card[] = [
		{
			id: 1,
			title: 'Lorem ipsum dolor sit amet 1',
			summary:
				'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.',
			date: '4월 전',
			comments: 11,
			views: 'Views',
			author: {
				name: 'Neo',
				avatar: 'https://picsum.photos/32/32?random=1'
			},
			likes: 34
		},
		{
			id: 2,
			title: 'Consectetur adipiscing elit 2',
			summary:
				'Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.',
			date: '6월 전',
			comments: 35,
			views: 'Views',
			author: {
				name: 'Luna',
				avatar: 'https://picsum.photos/32/32?random=2'
			},
			likes: 49
		},
		{
			id: 3,
			image: 'https://picsum.photos/400/225?random=3',
			title: 'Sed do eiusmod tempor 3',
			summary:
				'Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.',
			date: '2025년 7월 21일',
			comments: 1,
			views: 'Views',
			author: {
				name: 'Kai',
				avatar: 'https://picsum.photos/32/32?random=3'
			},
			likes: 39
		},
		{
			id: 4,
			image: 'https://picsum.photos/400/225?random=4',
			title: 'Incididunt ut labore et dolore 4',
			summary:
				'Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt.',
			date: '2025년 7월 19일',
			comments: 4,
			views: 'Views',
			author: {
				name: 'Mina',
				avatar: 'https://picsum.photos/32/32?random=4'
			},
			likes: 33
		},
		{
			id: 5,
			image: 'https://picsum.photos/400/225?random=5',
			title: 'Magna aliqua ut enim ad minim 5',
			summary:
				'Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem.',
			date: '3월 전',
			comments: 15,
			views: 'Views',
			author: {
				name: 'Jay',
				avatar: 'https://picsum.photos/32/32?random=5'
			},
			likes: 41
		},
		{
			id: 6,
			image: 'https://picsum.photos/400/225?random=6',
			title: 'Veniam quis nostrud 6',
			summary:
				'Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur?',
			date: '2월 전',
			comments: 9,
			views: 'Views',
			author: {
				name: 'Rin',
				avatar: 'https://picsum.photos/32/32?random=6'
			},
			likes: 27
		},
		{
			id: 7,
			image: 'https://picsum.photos/400/225?random=7',
			title: 'Exercitation ullamco laboris 7',
			summary:
				'Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?',
			date: '1년 전',
			comments: 22,
			views: 'Views',
			author: {
				name: 'Alex',
				avatar: 'https://picsum.photos/32/32?random=7'
			},
			likes: 65
		},
		{
			id: 8,
			image: 'https://picsum.photos/400/225?random=8',
			title: 'Nisi ut aliquip ex ea 8',
			summary:
				'At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident.',
			date: '5일 전',
			comments: 7,
			views: 'Views',
			author: {
				name: 'Sophia',
				avatar: 'https://picsum.photos/32/32?random=8'
			},
			likes: 59
		},
		{
			id: 9,
			image: 'https://picsum.photos/400/225?random=9',
			title: 'Commodo consequat duis 9',
			summary:
				'Similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio.',
			date: '2주 전',
			comments: 13,
			views: 'Views',
			author: {
				name: 'Ethan',
				avatar: 'https://picsum.photos/32/32?random=9'
			},
			likes: 45
		}
	];
</script>

<div class="min-h-screen">
	<div class=" grid grid-cols-1 gap-x-6 gap-y-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
		{#each cards as card (card.id)}
			<PostCard
				image={card.image}
				title={card.title}
				summary={card.summary}
				date={card.date}
				comments={card.comments}
				views={card.views}
				author_name={card.author.name}
				author_avatar={card.author.avatar}
				likes={card.likes}
			/>
		{/each}
	</div>
</div>
