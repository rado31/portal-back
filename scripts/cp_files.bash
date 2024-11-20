cp_files() {
	rsync -ah --info=progress2 "$SOURCE_PATH/changes/movies/*" > "$TARGET_PATH/movies"
	rsync -ah --info=progress2 "$SOURCE_PATH/changes/musics/*" > "$TARGET_PATH/musics"
	rsync -ah --info=progress2 "$SOURCE_PATH/changes/books/*" > "$TARGET_PATH/books"
}
