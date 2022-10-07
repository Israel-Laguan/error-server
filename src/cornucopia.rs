// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub mod types {  }#[allow(clippy::all, clippy::pedantic)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub mod queries { pub mod create_error { use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient; #[derive(Debug)]
            pub struct InsertErrorParams<'a> { pub app_id : &'a str,pub user_id : &'a str,pub message : &'a str,pub location : &'a str,pub context : &'a str,pub trace : &'a str }  pub fn insert_error() -> InsertErrorStmt {
                InsertErrorStmt(cornucopia_async::private::Stmt::new("INSERT INTO errors (app_id, user_id, message, location, context, trace)
    VALUES ($1, $2, $3, $4, $5, $6)"))
            }
            pub struct InsertErrorStmt(cornucopia_async::private::Stmt);
            impl InsertErrorStmt {pub async fn bind<'a, C: GenericClient>(&'a mut self, client: &'a  C, app_id : &'a &'a str,user_id : &'a &'a str,message : &'a &'a str,location : &'a &'a str,context : &'a &'a str,trace : &'a &'a str) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[app_id,user_id,message,location,context,trace]).await
            }
        }impl <'a, C: GenericClient + Send + Sync> cornucopia_async::Params<'a, InsertErrorParams<'a>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>>, C> for InsertErrorStmt  { 
                        fn params(&'a mut self, client: &'a  C, params: &'a InsertErrorParams<'a>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>> {
                            Box::pin(self.bind(client, &params.app_id,&params.user_id,&params.message,&params.location,&params.context,&params.trace))
                        }
                    }
                     }
pub mod read_errors { use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient; 
#[derive(Debug)]
            pub struct ErrorsQueriedParams<'a> { pub app_id : &'a str,pub query : &'a str,pub limit : i64,pub offset : i64 } 
            pub struct TimeOffsetDateTimeQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a  C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> time::OffsetDateTime,
                mapper: fn(time::OffsetDateTime) -> T,
            }
            impl<'a, C, T:'a, const N: usize> TimeOffsetDateTimeQuery<'a, C, T, N> where C: GenericClient {
                pub fn map<R>(self, mapper: fn(time::OffsetDateTime) -> R) -> TimeOffsetDateTimeQuery<'a,C,R,N> {
                    TimeOffsetDateTimeQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
            
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
            
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
            
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
            
                pub async fn iter(
                    self,
                ) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
#[derive( Debug, Clone, PartialEq,)] pub struct ErrorsQueried { pub id : uuid::Uuid,pub user_id : String,pub message : String,pub location : String,pub context : String,pub trace : String,pub created_at : time::OffsetDateTime }pub struct ErrorsQueriedBorrowed<'a> { pub id : uuid::Uuid,pub user_id : &'a str,pub message : &'a str,pub location : &'a str,pub context : &'a str,pub trace : &'a str,pub created_at : time::OffsetDateTime }
                impl<'a> From<ErrorsQueriedBorrowed<'a>> for ErrorsQueried {
                    fn from(ErrorsQueriedBorrowed { id,user_id,message,location,context,trace,created_at }: ErrorsQueriedBorrowed<'a>) -> Self {
                        Self { id,user_id: user_id.into(),message: message.into(),location: location.into(),context: context.into(),trace: trace.into(),created_at }
                    }
                }
            pub struct ErrorsQueriedQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a  C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> ErrorsQueriedBorrowed,
                mapper: fn(ErrorsQueriedBorrowed) -> T,
            }
            impl<'a, C, T:'a, const N: usize> ErrorsQueriedQuery<'a, C, T, N> where C: GenericClient {
                pub fn map<R>(self, mapper: fn(ErrorsQueriedBorrowed) -> R) -> ErrorsQueriedQuery<'a,C,R,N> {
                    ErrorsQueriedQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
            
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
            
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
            
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
            
                pub async fn iter(
                    self,
                ) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            } pub fn errors() -> ErrorsStmt {
                ErrorsStmt(cornucopia_async::private::Stmt::new("SELECT
    created_at
FROM
    errors
WHERE
    app_id = $1"))
            }
            pub struct ErrorsStmt(cornucopia_async::private::Stmt);
            impl ErrorsStmt {pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a  C, app_id : &'a &'a str) -> TimeOffsetDateTimeQuery<'a,C, time::OffsetDateTime, 1> {
                TimeOffsetDateTimeQuery {
                    client,
                    params: [app_id],
                    stmt: &mut self.0,
                    extractor: |row| { row.get(0) },
                    mapper: |it| { it },
                }
            }
        }
pub fn errors_queried() -> ErrorsQueriedStmt {
                ErrorsQueriedStmt(cornucopia_async::private::Stmt::new("SELECT
    id,
    user_id,
    message,
    location,
    context,
    trace,
    created_at
FROM
    errors
WHERE ( app_id = $1)
OR (
    message LIKE CONCAT($2::text, '%%') OR
    location  LIKE CONCAT($2::text, '%%')
)
ORDER BY created_at DESC
LIMIT $3
OFFSET $4"))
            }
            pub struct ErrorsQueriedStmt(cornucopia_async::private::Stmt);
            impl ErrorsQueriedStmt {pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a  C, app_id : &'a &'a str,query : &'a &'a str,limit : &'a i64,offset : &'a i64) -> ErrorsQueriedQuery<'a,C, ErrorsQueried, 4> {
                ErrorsQueriedQuery {
                    client,
                    params: [app_id,query,limit,offset],
                    stmt: &mut self.0,
                    extractor: |row| { ErrorsQueriedBorrowed {id: row.get(0),user_id: row.get(1),message: row.get(2),location: row.get(3),context: row.get(4),trace: row.get(5),created_at: row.get(6)} },
                    mapper: |it| { <ErrorsQueried>::from(it) },
                }
            }
        }impl <'a, C: GenericClient> cornucopia_async::Params<'a, ErrorsQueriedParams<'a>, ErrorsQueriedQuery<'a, C, ErrorsQueried, 4>, C> for ErrorsQueriedStmt  { 
                    fn params(&'a mut self, client: &'a  C, params: &'a ErrorsQueriedParams<'a>) -> ErrorsQueriedQuery<'a, C, ErrorsQueried, 4> {
                        self.bind(client, &params.app_id,&params.query,&params.limit,&params.offset)
                    }
                } } }