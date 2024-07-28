-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
-- sessions テーブルにインデックスを追加
CREATE INDEX idx_session_token ON sessions(session_token);

-- tow_trucks テーブルにインデックスを追加
CREATE INDEX idx_tow_truck_driver_id ON tow_trucks(driver_id);

-- locations テーブルにインデックスを追加
CREATE INDEX idx_location_tow_truck_id ON locations(tow_truck_id);

-- dispatchers テーブルにインデックスを追加
CREATE INDEX idx_dispatcher_user_id ON dispatchers(user_id);

-- orders テーブルにインデックスを追加
CREATE INDEX idx_order_client_id ON orders(client_id);
CREATE INDEX idx_order_dispatcher_id ON orders(dispatcher_id);
CREATE INDEX idx_order_tow_truck_id ON orders(tow_truck_id);

-- process_order_procedure プロシージャを作成
DELIMITER //
CREATE PROCEDURE process_order_procedure(
    IN p_order_id INT,
    IN p_dispatcher_id INT,
    IN p_tow_truck_id INT,
    IN p_completed_time DATETIME,
    IN p_new_tow_truck_status VARCHAR(50)
)
BEGIN
    INSERT INTO completed_orders (order_id, tow_truck_id, completed_time)
    VALUES (p_order_id, p_tow_truck_id, p_completed_time);

    UPDATE orders
    SET dispatcher_id = p_dispatcher_id, tow_truck_id = p_tow_truck_id, status = 'dispatched'
    WHERE id = p_order_id;

    UPDATE tow_trucks
    SET status = p_new_tow_truck_status
    WHERE id = p_tow_truck_id;
END //
DELIMITER ;

CREATE INDEX idx_area_id ON nodes(area_id);
CREATE INDEX idx_area_id_id ON nodes(area_id, id);
