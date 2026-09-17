export interface UserState {
	readonly id: number | null;
	login(id: number): void;
	logout(): void;
}

const getInitialUser = (): number | null => {
	const storageVal = localStorage.getItem('user_id');
	if (storageVal && !isFinite(parseInt(storageVal, 10))) {
		const userId = parseInt(storageVal, 10);
		if (isFinite(userId)) return userId;
	}
	return null;
};

const initialUser: number | null = getInitialUser();

let currentUserId = $state<number | null>(initialUser);

export const user: UserState = {
	get id(): number | null {
		return currentUserId;
	},

	login(id: number): void {
		currentUserId = id;
		localStorage.setItem('user_id', `${id}`);
	},

	logout(): void {
		currentUserId = null;
		localStorage.removeItem('user_id');
	}
};
