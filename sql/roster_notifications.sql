CREATE TABLE roster_notifications (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    table_name VARCHAR(32) NOT NULL,        -- 'controllers' | 'visits'
    row_pk BIGINT UNSIGNED NOT NULL,        -- controllers.cid or visits.id
    operation ENUM('INSERT','UPDATE','DELETE') NOT NULL,
    old_value JSON NULL,                    -- NULL for INSERT
    new_value JSON NULL,                    -- NULL for DELETE
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    processed_at TIMESTAMP NULL DEFAULT NULL,
    INDEX idx_roster_notifications_unprocessed (processed_at, id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- controllers: only log when `facility` actually changes.
-- `<=>` is MySQL's NULL-safe equality operator, so a change to/from NULL is
-- also caught (a plain `<>` comparison would miss NULL transitions).
DELIMITER $$

CREATE TRIGGER trg_controllers_facility_change
AFTER UPDATE ON controllers
FOR EACH ROW
BEGIN
    IF NOT (OLD.facility <=> NEW.facility) THEN
        INSERT INTO roster_notifications (table_name, row_pk, operation, old_value, new_value)
        VALUES (
            'controllers',
            NEW.cid,
            'UPDATE',
            JSON_OBJECT('facility', OLD.facility),
            JSON_OBJECT('facility', NEW.facility)
        );
    END IF;
END$$

DELIMITER ;

-- visits: log every insert/update/delete unconditionally.
-- Note: DELETE triggers only have access to OLD, never NEW.
DELIMITER $$

CREATE TRIGGER trg_visits_insert
AFTER INSERT ON visits
FOR EACH ROW
BEGIN
    INSERT INTO roster_notifications (table_name, row_pk, operation, old_value, new_value)
    VALUES (
        'visits', NEW.id, 'INSERT', NULL,
        JSON_OBJECT('id', NEW.id, 'cid', NEW.cid, 'facility', NEW.facility,
                     'created_at', NEW.created_at, 'updated_at', NEW.updated_at)
    );
END$$

CREATE TRIGGER trg_visits_update
AFTER UPDATE ON visits
FOR EACH ROW
BEGIN
    INSERT INTO roster_notifications (table_name, row_pk, operation, old_value, new_value)
    VALUES (
        'visits', NEW.id, 'UPDATE',
        JSON_OBJECT('id', OLD.id, 'cid', OLD.cid, 'facility', OLD.facility,
                     'created_at', OLD.created_at, 'updated_at', OLD.updated_at),
        JSON_OBJECT('id', NEW.id, 'cid', NEW.cid, 'facility', NEW.facility,
                     'created_at', NEW.created_at, 'updated_at', NEW.updated_at)
    );
END$$

CREATE TRIGGER trg_visits_delete
AFTER DELETE ON visits
FOR EACH ROW
BEGIN
    INSERT INTO roster_notifications (table_name, row_pk, operation, old_value, new_value)
    VALUES (
        'visits', OLD.id, 'DELETE',
        JSON_OBJECT('id', OLD.id, 'cid', OLD.cid, 'facility', OLD.facility,
                     'created_at', OLD.created_at, 'updated_at', OLD.updated_at),
        NULL
    );
END$$

DELIMITER ;
