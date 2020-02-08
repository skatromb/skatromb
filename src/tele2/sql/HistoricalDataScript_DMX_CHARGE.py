from datetime import date


ENV = 'PRD2'  # 'DEV'
if ENV == 'DEV':
    Layer = 'DDS'
    Postfix_prc = '_LIVADNYY'
    Postfix_tbl = '_2'
else:
    Layer = 'MDS'
    Postfix_prc = ''
    Postfix_tbl = ''


def generate_sql_for_month(year: date.year, month: date.month):

    def call_procedure_without_params(procedure_name: str, env: str = ENV, layer: str = Layer, postfix: str = Postfix_prc) -> str:
        call_statement = 'CALL ' + env + '_' + layer + '.' + procedure_name + postfix
        return call_statement

    # print('\n-- ' + str(year) + '-' + str(month) + ' ' + '-' * 64 + '\n')

    # DMX_CHARGE за неделю
    for day in range(18, 11, -1):
        print(call_procedure_without_params('LOAD_DMX_CHARGE_DATE') +
              "(7273889, date'2019-12-" + str(day) + "');")

    # Собираем статистику
    # print('COLLECT STATS ON ' + ENV + '_' + Layer + '.DMX_CHARGE' + Postfix_tbl + ';')

    # Блок месячных расчётов
    # print()
    params = "(7273889, date'" + date(year, month, 1).isoformat() + "');"

    print(call_procedure_without_params('LOAD_DMX_CHARGE_DATE') + params)

    # Удаляем информацию за рассчитанный месяц
    # print()
    # date_str = "'" + date(year, month, 1).isoformat() + "'"


# 2019 с декабря
for Month in range(12, 0, -1):
    generate_sql_for_month(year=2019, month=Month)

# 2018 до февраля
for Month in range(12, 1, -1):
    generate_sql_for_month(year=2018, month=Month)