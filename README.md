src/game içinde bulunan animation_ik.txt gibi txt dosyalarına ve docs içindeki şeylere bakmanıza gerek yok .
ben kodları birleştirmek için command yazıyom , ondan dolayı oluşuyor.
find player enemy health stamina  -name "*.rs" ! -path "*/target/*" ! -path "*/.git/*" -exec sh -c 'echo "=== FILE: {} ==="; cat "{}"; echo ""' \; > kodlar.txt
bi ara docs sistemi getireyim diye düşünmüştüm , sonra vazgeçtim , kodu anlamak istiyorsan kodu okuycan abi.
