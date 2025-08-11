import * as v from 'valibot';

export function createPostSchema() {
	return v.object({
		title: v.pipe(
			v.string(),
			v.minLength(1, '제목은 필수입니다.'),
			v.maxLength(80, '제목은 80자를 초과할 수 없습니다.')
		),
		content: v.pipe(v.string(), v.minLength(1, '내용은 필수입니다.')),
		slug: v.pipe(
			v.string(),
			v.minLength(1, '슬러그는 필수입니다.'),
			v.maxLength(80, '슬러그는 80자를 초과할 수 없습니다.'),
			v.regex(
				/^[^\s\/\?#\[\]@!$&'()*+,;=]+$/,
				'슬러그에는 공백이나 URL에 사용할 수 없는 특수문자를 포함할 수 없습니다.'
			)
		),
		summary: v.optional(v.pipe(v.string(), v.maxLength(500, '요약은 500자를 초과할 수 없습니다.'))),
		tags: v.optional(
			v.pipe(
				v.string(),
				v.transform((input) => {
					const tagArray = input
						.split(',')
						.map((tag) => tag.trim())
						.filter((tag) => tag);
					return tagArray;
				}),
				v.maxLength(8, '태그는 최대 8개까지 입력할 수 있습니다.')
			)
		)
	});
}

export type PostData = v.InferInput<ReturnType<typeof createPostSchema>>;
