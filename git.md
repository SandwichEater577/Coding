 ============================================
#   SCIAGA GIT — PSOISK SPRAWDZIAN
# ============================================
# otworz w terminalu: cat sciaga-git.md
# lub: less sciaga-git.md  (q zeby wyjsc)
# ============================================


# ── 1. TWORZENIE REPO ────────────────────────

git init                          # nowe lokalne repo
git clone <url>                   # sklonuj istniejace repo
git remote add origin <url>       # podepnij zdalne repo
git remote -v                     # sprawdz remotes

# podstawowy workflow:
git status                        # co sie zmienilo
git add .                         # dodaj wszystko
git add plik.txt                  # lub konkretny plik
git commit -m "opis zmian"        # zapisz commit
git push origin main              # wyslij na serwer
git pull origin main              # pobierz zmiany


# ── 2. GALĘZIE (BRANCHES) ────────────────────

git branch                        # lista galezi
git branch nowa-galaz             # stworz galaz
git checkout nowa-galaz           # przejdz na galaz
git checkout -b nowa-galaz        # stworz + przejdz (skrot!)
git branch -d nazwa               # usun galaz


# ── 3. GIT MERGE ─────────────────────────────
#
#  main:    A──B──C──────M        <- M = merge commit
#                  \    /
#  feature:         D──E
#
# Lacze dwie galęzie, tworzy nowy "merge commit"
# Historia ZACHOWANA — widac kiedy byl branch

git checkout main
git merge feature-branch          # wciagnij feature do main

# jesli konflikt → patrz punkt 5!


# ── 4. GIT REBASE ────────────────────────────
#
#  PRZED:
#  main:    A──B──C
#                  \
#  feature:         D──E
#
#  PO rebase:
#  main:    A──B──C──D'──E'       <- historia liniowa
#
# Przepisuje historie — commity laduja na czubku main
# Historia LINIOWA i czystsza
# !! NIGDY nie rób rebase na galezi publicznej/main !!

git checkout feature-branch
git rebase main                   # przenieś commity na czubek main

# po konflikcie podczas rebase:
git rebase --continue             # kontynuuj
git rebase --abort                # anuluj caly rebase


# ── 5. KONFLIKTY — ROZWIĄZANIE ───────────────
#
# Konflikt wygląda tak w pliku:
#
# <<<<<<< HEAD
# twoja wersja (obecna galaz)
# =======
# wersja z drugiej galezi
# >>>>>>> feature-branch
#
# KROKI:
# 1. otwórz plik, znajdz znaczniki
# 2. zostaw TYLKO to co chcesz, usun <<<, ===, >>>
# 3. zapisz plik

git add plik.txt                  # oznacz jako rozwiazany
git merge --continue              # po merge
git rebase --continue             # po rebase
git commit                        # zatwierdz (merge tworzy commit)

# chcesz porzucic merge:
git merge --abort


# ── 6. PULL REQUEST (PR) ─────────────────────
#
# PR = propozycja wciagniecia zmian z galezi do galezi
# Robi sie przez GitHub (nie terminal!)
#
# WORKFLOW:
# 1. stworz galaz
git checkout -b moja-funkcja

# 2. zrob zmiany i commituj
git add .
git commit -m "dodałem nową funkcję"

# 3. wyslij galaz na GitHub
git push origin moja-funkcja

# 4. wejdz na GitHub → "Compare & pull request"
# 5. opisz co zrobiłes → "Create pull request"
# 6. ktos robi code review
# 7. klikasz "Merge pull request" na GitHubie


# ── 7. GIT CHERRY-PICK ───────────────────────
#
# Bierze JEDEN konkretny commit z innej galezi
# i aplikuje go na obecnej galezi
# Nie merguje calej galezi — tylko wybrany commit!

git log --oneline                 # znajdz hash commita
# przyklad wyniku:
# abc1234 naprawiono buga w logowaniu
# def5678 dodano strone glowna

git cherry-pick abc1234           # wez tylko ten commit

# jesli konflikt:
git cherry-pick --continue        # po rozwiazaniu
git cherry-pick --abort           # anuluj


# ── 8. PRZYDATNE DODATKOWE ───────────────────

git log --oneline --graph         # historia jako drzewo
git diff                          # podglad zmian
git stash                         # schowaj zmiany na pozniej
git stash pop                     # przywroc schowane zmiany
git reset --hard HEAD             # cofnij wszystkie zmiany (UWAGA!)
git show abc1234                  # pokaz co zrobil dany commit


# ============================================
#  MERGE vs REBASE — kiedy co uzyc?
#
#  MERGE:
#  ✓ galaz publiczna / wspolna z innymi
#  ✓ chcesz zachowac pelna historie
#  ✓ feature branch → main
#
#  REBASE:
#  ✓ lokalna galaz, nie udostepniona
#  ✓ chcesz czystą, liniową historię
#  ✗ NIE na main / galęziach wspolnych!
# ============================================
