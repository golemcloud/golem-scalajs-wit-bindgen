
                // Generated by golem-scalajs-wit-bindgen
                package tests

                import scala.scalajs.js
                import scala.scalajs.js.JSConverters._

                
                sealed trait T20 extends js.Object {
                    
                }
                object T20 {
                    def apply(): T20 = {
                        

                        new T20 {
                            
                        }
                    }
                }
            

                sealed trait T21 extends js.Object {
                    val a: Integer
                }
                object T21 {
                    def apply(a: Integer): T21 = {
                        val a0: Integer = a

                        new T21 {
                            val a: Integer = a0
                        }
                    }
                }
            

                sealed trait T22 extends js.Object {
                    val a: Integer
                }
                object T22 {
                    def apply(a: Integer): T22 = {
                        val a0: Integer = a

                        new T22 {
                            val a: Integer = a0
                        }
                    }
                }
            

                sealed trait T23 extends js.Object {
                    val a: Integer
val b: Long
                }
                object T23 {
                    def apply(a: Integer, b: Long): T23 = {
                        val a0: Integer = a
val b0: Long = b

                        new T23 {
                            val a: Integer = a0
val b: Long = b0
                        }
                    }
                }
            

                sealed trait T24 extends js.Object {
                    val a: Integer
val b: Long
                }
                object T24 {
                    def apply(a: Integer, b: Long): T24 = {
                        val a0: Integer = a
val b0: Long = b

                        new T24 {
                            val a: Integer = a0
val b: Long = b0
                        }
                    }
                }
            

                sealed trait T25 extends js.Object {
                    val x: Integer
                }
                object T25 {
                    def apply(x: Integer): T25 = {
                        val x0: Integer = x

                        new T25 {
                            val x: Integer = x0
                        }
                    }
                }
            

                sealed trait Record extends js.Object {
                    
                }
                object Record {
                    def apply(): Record = {
                        

                        new Record {
                            
                        }
                    }
                }
            

                
                sealed trait T33 extends js.Object { self =>
                    type Type

                    val tag: String
                    val `val`: js.UndefOr[Type]
                }

                object T33 {
                    
                            val a = new T33 {
                                type Type = Nothing
                                
                                val tag: String = "a"
                                
                            }
                        
                }
            

                sealed trait T34 extends js.Object { self =>
                    type Type

                    val tag: String
                    val `val`: js.UndefOr[Type]
                }

                object T34 {
                    
                            val a = new T34 {
                                type Type = Nothing
                                
                                val tag: String = "a"
                                
                            }
                        

                            val b = new T34 {
                                type Type = Nothing
                                
                                val tag: String = "b"
                                
                            }
                        
                }
            

                sealed trait T35 extends js.Object { self =>
                    type Type

                    val tag: String
                    val `val`: js.UndefOr[Type]
                }

                object T35 {
                    
                            val a = new T35 {
                                type Type = Nothing
                                
                                val tag: String = "a"
                                
                            }
                        

                            val b = new T35 {
                                type Type = Nothing
                                
                                val tag: String = "b"
                                
                            }
                        
                }
            

                sealed trait T36 extends js.Object { self =>
                    type Type

                    val tag: String
                    val `val`: js.UndefOr[Type]
                }

                object T36 {
                    
                            val a = new T36 {
                                type Type = Nothing
                                
                                val tag: String = "a"
                                
                            }
                        

                            def b(value: Integer) = new T36 {
                                type Type = Integer
                                
                                val tag: String = "b"
                                override val `val`: js.UndefOr[Type] = value
                            }
                        
                }
            

                sealed trait T37 extends js.Object { self =>
                    type Type

                    val tag: String
                    val `val`: js.UndefOr[Type]
                }

                object T37 {
                    
                            val a = new T37 {
                                type Type = Nothing
                                
                                val tag: String = "a"
                                
                            }
                        

                            def b(value: WitOption[Integer]) = new T37 {
                                type Type = WitOption[Integer]
                                
                                val tag: String = "b"
                                override val `val`: js.UndefOr[Type] = value
                            }
                        
                }
            

                trait Api {
                    type WitResult[+Ok, +Err] = Ok
                    object WitResult {
                        def ok[Ok](value: Ok): WitResult[Ok, Nothing] = value

                        def err[Err](value: Err): WitResult[Nothing, Err] = throw js.JavaScriptException(value)

                        val unit: WitResult[Unit, Nothing] = ()
                    }

                    type WitOption[+A] = js.UndefOr[A]
                    object WitOption {
                        def some[A](value: A): WitOption[A] = value

                        val none: WitOption[Nothing] = js.undefined

                        def fromOption[A](option: Option[A]) =
                        option match {
                            case Some(value) => value.asInstanceOf[js.UndefOr[A]]
                            case None        => js.undefined
                        }
                    }

                    type WitList[A] = js.Array[A]
                    object WitList {
                        def fromList[A](list: List[A]): WitList[A] = list.toJSArray
                    }
                    
                    
                }
            