#%%
import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns
from sklearn.cluster import KMeans
import os



path = os.getcwd()

df = pd.read_csv(path + "/mall_customers/Mall_Customers.csv",
                 encoding="UTF-8",
                 engine='python')

df.head()
#%%

df.tail()

#%%

_df = df
_df = _df.replace("Male", 0)
_df = _df.replace("Female", 1)
# データの雰囲気をつかむ
_df.describe()

#%%
sns.distplot(df['Spending Score (1-100)'])

#%%
sns.jointplot(x='Spending Score (1-100)', y='Annual Income (k$)', data=df)

#%%
man = df[df["Gender"] == "Male"]
sns.jointplot(x='Spending Score (1-100)', y='Annual Income (k$)', data=man)

woman = df[df["Gender"] == "Female"]
sns.jointplot(x='Spending Score (1-100)', y='Annual Income (k$)', data=woman)

#%%
sns.jointplot(x='Spending Score (1-100)', y='Age', data=df)
sns.jointplot(x='Spending Score (1-100)', y='Age', data=woman)
sns.jointplot(x='Spending Score (1-100)', y='Age', data=man)


#%%
sns.jointplot(x='Annual Income (k$)', y='Age', data=df)


#%%
def clustering(k, df):
    _df = df.replace("Male", 0)
    _df = _df.replace("Female", 1)
    pred = KMeans(n_clusters=k).fit_predict(_df)
    _df['cluster_id'] = pred

    sns.relplot(x='Spending Score (1-100)',
                y='Annual Income (k$)', data=_df,
                col="Gender", hue='cluster_id')

#%%
clustering(3, df)
#%%
clustering(4, df)
#%%
clustering(5, df)