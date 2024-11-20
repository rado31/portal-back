delete_files() {
	MOVIES=$(jq -r '.movies | join(",")' "$JSON_FILE")
	MUSICS=$(jq -r '.musics | join(",")' "$JSON_FILE")
	BOOKS=$(jq -r '.books | join(",")' "$JSON_FILE")

	eval rm -rf "$TARGET_PATH/movies/{$MOVIES}"
	eval rm "$TARGET_PATH/musics/{$MUSICS}.mp3"
	eval rm "$TARGET_PATH/books/{$BOOKS}.pdf"
}