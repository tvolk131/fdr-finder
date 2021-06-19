import {Checkbox, FormGroup, FormControlLabel, FormLabel, Paper} from '@material-ui/core';
import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import {useState} from 'react';

const useStyles = makeStyles({
  root: {
    padding: '15px 5px 5px 15px'
  }
});

const AdvancedSearchMenu = () => {
  const [interviewsChecked, setInterviewsChecked] = useState(false);
  const [presentationsChecked, setPresentationsChecked] = useState(false);
  const [callInsChecked, setCallInsChecked] = useState(false);
  const [roundtablesChecked, setRoundtablesChecked] = useState(false);

  const classes = useStyles();

  return (
    <Paper className={classes.root} elevation={10}>
      <FormGroup>
        <FormLabel>Show Format (Doesn't actually do anything yet)</FormLabel>
        <FormControlLabel
          control={
            <Checkbox
              checked={interviewsChecked}
              onChange={(event) => setInterviewsChecked(event.target.checked)}
            />
          }
          label={'Interviews'}
        />
        <FormControlLabel
          control={
            <Checkbox
              checked={presentationsChecked}
              onChange={(event) => setPresentationsChecked(event.target.checked)}
            />
          }
          label={'Presentations'}
        />
        <FormControlLabel
          control={
            <Checkbox
              checked={callInsChecked}
              onChange={(event) => setCallInsChecked(event.target.checked)}
            />
          }
          label={'Call-Ins'}
        />
        <FormControlLabel
          control={
            <Checkbox checked={roundtablesChecked} onChange={(event) => setRoundtablesChecked(event.target.checked)}/>
          }
          label={'Roundtables'}
        />
      </FormGroup>
    </Paper>
  );
};

export default AdvancedSearchMenu;