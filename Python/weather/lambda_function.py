"""
OpenWeatherMapから現在の気象データを取得して
DynamoDBに保存する。
"""

import boto3
import requests
import json
import datetime
import decimal


def getWheather(city_name):
    API_KEY = "1908d3e034bb5cd0ef3a96fd01040309"  # xxxに自分のAPI Keyを入力。
    api = "http://api.openweathermap.org/data/2.5/weather?units=metric&q={city}&APPID={key}"

    url = api.format(city=city_name, key=API_KEY)
    print(url)

    response = requests.get(url)
    # APIレスポンスの表示
    jsonText = json.dumps(response.json(), indent=2)
    print(jsonText)

    return response.json(parse_float=decimal.Decimal)


def formatter(response):
    data = response
    # 表示用時刻
    unix = data["dt"]
    now = datetime.datetime.fromtimestamp(unix)

    # 保存するデータを作成
    item = {
        'city_name': data['name'],  # プライマリパーティションキー
        'timestamp': data["dt"],   # プライマリソートキー
        'datetime': now.strftime("%Y-%m-%dT%H:%M:%S"),
        'latitude': data['coord']['lat'],
        'longitude': data['coord']['lon'],
        'data': {
            'weather': data['weather'],
            'temp': data['main']['temp'],
            'humidity': data['main']['humidity'],
            'pressure': data['main']['pressure'],
            'clouds': data['clouds']['all'],
            'wind': data['wind']
        }
    }
    return item


def insert(items):
    # Initialize connection with database
    session = boto3.session.Session(
        region_name='ap-northeast-1',
        aws_access_key_id='AKIAIQ47YOEZWBRDLHRA',
        aws_secret_access_key='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'
    )
    dynamodb = session.resource('dynamodb')

    # tableのパーティション用
    now = datetime.datetime.now()
    ym = now.strftime("%Y%m")

    # Connect to db table
    table_name = 'current-weather.' + ym
    table = dynamodb.Table(table_name)

    for item in items:
        # 追加する
        response = table.put_item(
            TableName=table_name,
            Item=item
        )
        if response['ResponseMetadata']['HTTPStatusCode'] is not 200:
            print(response)
        else:
            print('Successed :', item['city_name'])
    return


def weather_api(cities):
    items = []

    for city_name in cities:
        response = getWheather(city_name)

        if response['cod'] is not 200:
            # 取得できなかった気象情報の処理
            print(city_name, ', status code :',
                  response['cod'], response['sys']['message'])
            continue
        item = formatter(response)
        print(item)
        items.append(item)

    insert(items)
    return True


def lambda_handler(event, context):
    # 都市と送信先を指定する。
    # OWMで指定可能な都市名は以下で確認する。
    # https://openweathermap.org/weathermap?basemap=map&cities=true&layer=temperature&lat=44.0079&lon=144.2487&zoom=12
    cities = [
        'Sapporo',
        'Tokyo',
        'Osaka-shi',
        'Okinawa',
        'Hong Kong',
        'New York',
        'City of London'
    ]
    weather_api(cities)

    return


if __name__ == "__main__":
    lambda_handler(None, None)