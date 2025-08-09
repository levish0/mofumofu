import * as v from 'valibot';

export function createSignupSchema() {
	return v.object({
		email: v.pipe(
			v.string(),
			v.minLength(1, '이메일을 입력해주세요'),
			v.email('올바른 이메일 형식이 아닙니다')
		),
		handle: v.pipe(
			v.string(),
			v.minLength(1, '핸들을 입력해주세요'),
			v.minLength(3, '핸들은 3자 이상이어야 합니다'),
			v.maxLength(20, '핸들은 20자 이하여야 합니다'),
			v.regex(/^[a-zA-Z0-9_]+$/, '핸들은 영문, 숫자, 언더스코어만 사용 가능합니다')
		),
		password: v.pipe(
			v.string(),
			v.minLength(1, '비밀번호를 입력해주세요'),
			v.minLength(8, '비밀번호는 8자 이상이어야 합니다'),
			v.maxLength(128, '비밀번호는 128자 이하여야 합니다'),
			v.regex(/(?=.*[a-z])/, '소문자를 포함해야 합니다'),
			v.regex(/(?=.*[A-Z])/, '대문자를 포함해야 합니다'),
			v.regex(/(?=.*\d)/, '숫자를 포함해야 합니다'),
			v.regex(/(?=.*[!@#$%^&*(),.?":{}|<>])/, '특수문자를 포함해야 합니다')
		)
	});
}

export type SignupInfo = v.InferInput<ReturnType<typeof createSignupSchema>>;