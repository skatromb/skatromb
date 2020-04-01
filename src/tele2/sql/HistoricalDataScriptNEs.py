from datetime import date
from calendar import Calendar


NE_SUBS_LAYER = 'DM'
LOAD_ID = '7273889'

ENV = 'PRD'  # 'PRD'
if ENV == 'DEV':
    Layer = 'DDS'
    Postfix = '_LIVADNYY'
else:
    Layer = 'MDS'
    Postfix = ''


def generate_sql_for_month(year: date.year, month: date.month):

    def call_procedure_without_params(procedure_name: str, env: str = ENV, layer: str = Layer, postfix: str = Postfix) -> str:
        call_statement = 'CALL ' + env + '_' + layer + '.' + procedure_name + postfix
        return call_statement

    print('\n-- ' + str(year) + '-' + str(month) + ' ' + '-' * 64 + '\n')

    # Блок генерации кода на каждый день месяца
    dates = Calendar()
    for day in [i for i in dates.itermonthdates(year, month) if i.month == month]:
        print(call_procedure_without_params('LOAD_NE_SUBS_REVENUE_DATE', env='PRD', layer=NE_SUBS_LAYER) +
              "(" + LOAD_ID + ", date'" + day.isoformat() + "');")

    print('COLLECT STATS ON ' + ENV + '_' + NE_SUBS_LAYER + '.NE_SUBS_REVENUE_DATE' + Postfix + ';')

    # Блок месячных расчётов
    print()
    params = "(date'" + date(year, month, 1).isoformat() + "', " + LOAD_ID + ");"

    print(call_procedure_without_params('LOAD_NETWORK_ELEMENT_MONTHLY_METRICS') + params)
    print(call_procedure_without_params('LOAD_SITE_MONTHLY_METRICS') + params)

    # Удаляем информацию за рассчитанный месяц
    print()
    date_str = "'" + date(year, month, 1).isoformat() + "'"
    print("DELETE " + ENV + "_DDS.NE_SUBS_REVENUE_DATE" + Postfix + " WHERE REPORT_DATE BETWEEN "
          "date" + date_str + " AND ADD_MONTHS(date" + date_str + ", 1) - 1;")


# 2019 с декабря
for Month in range(3, 2, -1):
    generate_sql_for_month(year=2019, month=Month)

# 2018 до февраля
# for Month in range(12, 1, -1):
#     generate_sql_for_month(year=2018, month=Month)
