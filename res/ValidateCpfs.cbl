       identification division.
       program-id. ValidadeCpfs.

       environment                division.
       configuration              section.
       special-names.
           decimal-point     is   comma.
       input-output section.
       file-control.
             select cpfs assign to cpfs-file
             organization is line sequential
             file status is fs-cpfs.            
 
       data division.
       file section.
       fd  cpfs.
       01  cpfs-reg               pic is x(14).

       working-storage section.
       77  cpfs-file              pic x(64) value "cpfs.txt".
       77  fs-cpfs                pic x(02) value spaces.
       77  error-message          pic x(64) value spaces.
       77  dig-bin                pic 9(02) comp-5.
       77  idx                    pic 9(08) comp-5.
       77  res                    pic 9(08) comp-5.
       77  summ                   pic 9(08) comp-5.
       77  remain                 pic 9(08) comp-5.
       
       01  cpf-num                pic 9(11) value zeros.
       01  filler redefines cpf-num.
           05 cpf-dig             pic 9 occurs 11.

       procedure division.
           open input cpfs
           if fs-cpfs <> "00"
              perform show-cpfs-file-error
              stop run
           end-if
          
           perform until exit
              read cpfs next
              if fs-cpfs <> "00"
                 exit perform
              end-if
      
              move cpfs-reg(1:3) to cpf-num(1:3)
              move cpfs-reg(5:3) to cpf-num(4:3)
              move cpfs-reg(9:3) to cpf-num(7:3)
              move zeros to cpf-num(10:2)

              perform first-digit
              perform second-digit
              
              if cpfs-reg(13:2) <> cpf-num(10:2)
                 display "cpf inválido: " cpfs-reg, " digitos calculados: " cpf-num(10:2)
              end-if

           end-perform

           close cpfs
           stop run
           .

       first-digit.
           move zeros to summ
           perform varying idx from 1 by 1 until idx > 9
               compute res = cpf-dig(idx) * (11 - idx)
               add res to summ
           end-perform
           move function rem(summ, 11) to remain
           if remain > 1
              compute cpf-dig(10) = 11 - remain
           end-if
           .

       second-digit.
           move zeros to summ
           perform varying idx from 1 by 1 until idx > 10
              compute res = cpf-dig(idx) * (12 - idx)
              add res to summ
           end-perform
           move function rem(summ, 11) to remain
           if remain > 1
              compute cpf-dig(11) = 11 - remain
           end-if
           .

       show-cpfs-file-error.
           copy "FileStat-Msgs.cpy" replacing stat by fs-cpfs, msg by error-message.
           display function trim(error-message) " '" function trim(cpfs-file) "', (status = " fs-cpfs ")"
           .
