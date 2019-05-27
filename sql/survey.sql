select *
from tr_reward_management as rm
    left JOIN tr_survey_answer_screening as sa1
    ON sa1.create_id =  rm.u_id AND sa1.st_id = 'ST100037'
    left JOIN tr_survey_answer as sa2
    ON sa2.create_id =  rm.u_id AND sa2.st_id = 'ST100038'
where rm.a_id = 'A8028410';


スクリーニングアンケートST100037
本アンケートST100038